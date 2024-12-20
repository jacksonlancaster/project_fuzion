using System;
using System.Collections.Generic;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Threading.Tasks;
using OpenProtocolInterpreter;
using OpenProtocolInterpreter.Alarm;
using OpenProtocolInterpreter.Communication;
using OpenProtocolInterpreter.Job;
using OpenProtocolInterpreter.KeepAlive;
using OpenProtocolInterpreter.ParameterSet;
using OpenProtocolInterpreter.Result;
using OpenProtocolInterpreter.Tightening;
using OpenProtocolInterpreter.Tool;
using OpenProtocolInterpreter.Vin;

namespace OpenProtocol_Server;

public class Tool : IDisposable
{
	public bool disposing = false;

	public ToolConfig config;

	public string Name;

	public string ResultPattern;

	public string Port;

	public int ResultDelay;

	private NetworkStream stream;

	public bool TOOL_ENABLED;

	public bool PSET_Selected_By_Integrator = false;

	public bool MID0061_Subscribed;

	public bool MID1201_Subscribed;

	public string _VIN = "";

	private int Selected_PSET_ID = 1;

	public List<PSET> PSETs;

	public string VIN
	{
		get
		{
			return _VIN;
		}
		set
		{
			_VIN = value;
			if (this.VIN_Changed != null)
			{
				this.VIN_Changed(_VIN);
			}
		}
	}

	public event VIN_Change VIN_Changed;

	public Tool(string Name, string Port, string ResultPattern, int ResultDelay, List<PSET> pSETs)
	{
		this.Name = Name;
		this.Port = Port;
		this.ResultDelay = ResultDelay;
		this.ResultPattern = ResultPattern;
		PSETs = pSETs;
	}

	public async Task Start()
	{
		Rundowns();
		await Task.Run(delegate
		{
			while (!disposing)
			{
				TcpListener tcpListener = null;
				MidInterpreter midInterpreter = new MidInterpreter();
				try
				{
					int port = int.Parse(Port);
					tcpListener = new TcpListener(IPAddress.Any, port);
					tcpListener.Start();
					byte[] array = new byte[256];
					string text = null;
					while (true)
					{
						Program.Log(Name + " Waiting for a connection... ");
						using TcpClient tcpClient = tcpListener.AcceptTcpClient();
						Program.Log(Name + " Connected!");
						text = null;
						stream = tcpClient.GetStream();
						stream.ReadTimeout = 30000;
						int count;
						while ((count = stream.Read(array, 0, array.Length)) != 0)
						{
							text = Encoding.ASCII.GetString(array, 0, count);
							midInterpreter.UseAllMessages(InterpreterMode.Controller);
							Mid mid = null;
							try
							{
								mid = midInterpreter.Parse(text);
							}
							catch (Exception value)
							{
								Program.Log($"{Name} error parsing: {value}");
							}
							Program.Log($"{Name} MID Recieved : {mid}");
							Mid mid2 = mid;
							Mid mid3 = mid2;
							if (!(mid3 is Mid0001))
							{
								if (!(mid3 is Mid0008))
								{
									if (!(mid3 is Mid9999))
									{
										if (!(mid3 is Mid0010))
										{
											if (!(mid3 is Mid0030))
											{
												if (!(mid3 is Mid0062))
												{
													if (!(mid3 is Mid0051))
													{
														if (!(mid3 is Mid0050))
														{
															if (!(mid3 is Mid0053))
															{
																if (!(mid3 is Mid0060))
																{
																	if (!(mid3 is Mid0070))
																	{
																		if (!(mid3 is Mid0014))
																		{
																			if (!(mid3 is Mid0018))
																			{
																				if (!(mid3 is Mid0042))
																				{
																					if (mid3 is Mid0043)
																					{
																						Mid0005 mid4 = new Mid0005(mid.Header.Mid);
																						byte[] array2 = mid4.PackBytesWithNul();
																						stream.Write(array2, 0, array2.Length);
																						stream.FlushAsync();
																						TOOL_ENABLED = true;
																						Program.Log(Name + " tool enabled");
																					}
																					else if (mid != null)
																					{
																						Mid0004 mid5 = new Mid0004(mid.Header.Mid, Error.NO_ERROR);
																						byte[] array3 = mid5.PackBytesWithNul();
																						stream.Write(array3, 0, array3.Length);
																						stream.FlushAsync();
																					}
																				}
																				else
																				{
																					Mid0005 mid6 = new Mid0005(mid.Header.Mid);
																					byte[] array4 = mid6.PackBytesWithNul();
																					stream.Write(array4, 0, array4.Length);
																					stream.FlushAsync();
																					TOOL_ENABLED = false;
																					Program.Log(Name + " tool disabled");
																				}
																			}
																			else
																			{
																				Selected_PSET_ID = ((Mid0018)mid).ParameterSetId;
																				Program.Log($"{Name} PSET {Selected_PSET_ID} selection command sent by Integrator");
																				PSET pSET = PSETs.Find((PSET e) => e.ID == Selected_PSET_ID);
																				if (pSET == null)
																				{
																					Program.Log($"{Name} PSET {Selected_PSET_ID} not found in controller; rejecting command");
																					Mid0004 mid7 = new Mid0004(Selected_PSET_ID, Error.PARAMETER_SET_CANNOT_BE_SET);
																					byte[] array5 = mid7.PackBytesWithNul();
																					stream.Write(array5, 0, array5.Length);
																					stream.FlushAsync();
																				}
																				else
																				{
																					Program.Log($"{Name} PSET {Selected_PSET_ID} found in controller");
																					Mid0005 mid8 = new Mid0005(mid.Header.Mid);
																					byte[] array6 = mid8.PackBytesWithNul();
																					stream.Write(array6, 0, array6.Length);
																					stream.FlushAsync();
																					Mid0015 mid9 = new Mid0015(Selected_PSET_ID, DateTime.Now);
																					byte[] array7 = mid9.PackBytesWithNul();
																					stream.Write(array7, 0, array7.Length);
																					stream.FlushAsync();
																					PSET_Selected_By_Integrator = true;
																				}
																			}
																		}
																		else
																		{
																			Mid0005 mid10 = new Mid0005(mid.Header.Mid);
																			byte[] array8 = mid10.PackBytesWithNul();
																			stream.Write(array8, 0, array8.Length);
																			stream.FlushAsync();
																			Program.Log(Name + " subscribe PSET selected");
																			SendMID0015(stream);
																		}
																	}
																	else
																	{
																		Mid0005 mid11 = new Mid0005(mid.Header.Mid);
																		byte[] array9 = mid11.PackBytesWithNul();
																		stream.Write(array9, 0, array9.Length);
																		stream.FlushAsync();
																		SendMID0071(stream);
																	}
																}
																else
																{
																	Mid0005 mid12 = new Mid0005(mid.Header.Mid);
																	byte[] array10 = mid12.PackBytesWithNul();
																	stream.Write(array10, 0, array10.Length);
																	stream.FlushAsync();
																	MID0061_Subscribed = true;
																}
															}
														}
														else
														{
															Mid0005 mid13 = new Mid0005(mid.Header.Mid);
															byte[] array11 = mid13.PackBytesWithNul();
															stream.Write(array11, 0, array11.Length);
															stream.FlushAsync();
															VIN = ((Mid0050)mid).VinNumber;
														}
													}
													else
													{
														Mid0005 mid14 = new Mid0005(mid.Header.Mid);
														byte[] array12 = mid14.PackBytesWithNul();
														stream.Write(array12, 0, array12.Length);
														stream.FlushAsync();
														VIN_Changed += Core_VIN_Changed;
														SendMID0052(stream);
													}
												}
											}
											else
											{
												Mid0031 mid15 = new Mid0031();
												byte[] array13 = mid15.PackBytesWithNul();
												stream.Write(array13, 0, array13.Length);
												stream.FlushAsync();
											}
										}
										else
										{
											Mid0011 mid16 = new Mid0011();
											foreach (PSET current in PSETs)
											{
												mid16.ParameterSets.Add(current.ID);
											}
											byte[] array14 = mid16.PackBytesWithNul();
											stream.Write(array14, 0, array14.Length);
											stream.FlushAsync();
										}
									}
									else
									{
										Program.Log(Name + " KeepAlive ack");
										stream.Write(array, 0, count);
										stream.FlushAsync();
									}
								}
								else
								{
									Program.Log(Name + " Mid0008 recieved from Integrator: " + ((Mid0008)mid).SubscriptionMid);
									Mid0005 mid17 = new Mid0005(mid.Header.Mid);
									byte[] array15 = mid17.PackBytesWithNul();
									stream.Write(array15, 0, array15.Length);
									stream.FlushAsync();
									switch (((Mid0008)mid).SubscriptionMid)
									{
									case "0015":
										SendMID0015(stream);
										break;
									case "0071":
										SendMID0071(stream);
										break;
									case "0052":
										VIN_Changed += Core_VIN_Changed;
										break;
									case "1201":
									{
										MID1201_Subscribed = true;
										Mid0005 mid18 = new Mid0005(1201);
										byte[] array16 = mid18.PackBytesWithNul();
										stream.Write(array16, 0, array16.Length);
										stream.FlushAsync();
										SendMID1201(stream);
										break;
									}
									}
								}
							}
							else
							{
								Mid0002 mid19 = new Mid0002(1, 1, Name);
								byte[] array17 = mid19.PackBytesWithNul();
								stream.Write(array17, 0, array17.Length);
								stream.FlushAsync();
							}
							text = text.ToUpper();
							byte[] bytes = Encoding.ASCII.GetBytes(text);
						}
					}
				}
				catch (SocketException ex)
				{
					Program.Log("SocketException: {0}", ex);
				}
				catch (Exception ex2)
				{
					Program.Log($"{Name} Exception: {0}", ex2);
				}
				finally
				{
					tcpListener.Stop();
				}
			}
		});
	}

	private void SendMID1202(bool fail, NetworkStream stream)
	{
		Mid1202 mid1202 = new Mid1202();
		mid1202.MessageNumber = 1;
		mid1202.TotalNumberOfMessages = 1;
		List<VariableDataField> fields = new List<VariableDataField>();
		VariableDataField variableDataField = new VariableDataField();
		variableDataField.ParameterId = 3;
		variableDataField.RealValue = "fdgdfgdf";
		fields.Add(variableDataField);
		mid1202.VariableDataFields = fields;
		mid1202.NumberOfDataFields = 2;
	}

	private void SendMID1201(NetworkStream stream)
	{
	}

	private void SendMID0052(NetworkStream stream)
	{
		Mid0052 mid0052 = new Mid0052(VIN);
		byte[] ack = mid0052.PackBytesWithNul();
		stream.Write(ack, 0, ack.Length);
		stream.FlushAsync();
	}

	private void SendMID0061(NetworkStream stream)
	{
		throw new NotImplementedException();
	}

	private void SendMID0015(NetworkStream stream)
	{
		Mid0015 mid0015 = new Mid0015(Selected_PSET_ID, DateTime.Now);
		byte[] ack0015 = mid0015.PackBytesWithNul();
		stream.Write(ack0015, 0, ack0015.Length);
		stream.FlushAsync();
	}

	private void SendMID0071(NetworkStream stream)
	{
		Mid0071 mid0071 = new Mid0071("0", controllerReadyStatus: true, toolReadyStatus: true, DateTime.Now);
		byte[] ack0071 = mid0071.PackBytesWithNul();
		stream.Write(ack0071, 0, ack0071.Length);
		stream.FlushAsync();
	}

	private void Core_VIN_Changed(string VIN)
	{
		Mid0052 mid0052 = new Mid0052(VIN);
		byte[] ack = mid0052.PackBytesWithNul();
		stream.Write(ack, 0, ack.Length);
		stream.FlushAsync();
	}

	public void Dispose()
	{
		disposing = true;
	}

	public async Task Rundowns()
	{
		await Task.Run(delegate
		{
			int num = 0;
			int num2 = 0;
			string resultPattern = ResultPattern;
			num2 = resultPattern.Length;
			long num3 = 1L;
			while (!disposing)
			{
				if (stream != null && stream.Socket.Connected)
				{
					if (PSET_Selected_By_Integrator && MID0061_Subscribed)
					{
						bool flag = true;
						if (TOOL_ENABLED)
						{
							char c = resultPattern.Substring(num, 1)[0];
							if (c == 'R')
							{
								Random random = new Random();
								int num4 = random.Next();
								int num5 = num4 % 1;
								int num6 = num4 % 2;
								if (num4 % 2 == 0)
								{
									c = 'P';
									Program.Log(Name + " Random Result OK");
								}
								else
								{
									c = 'F';
									Program.Log(Name + " Random Result NOK");
								}
							}
							switch (c)
							{
							case 'P':
							{
								Program.Log(Name + " Send Result OK");
								PSET pSET2 = PSETs.Find((PSET e) => e.ID == Selected_PSET_ID);
								Mid0061 mid2 = new Mid0061(1);
								mid2.AngleMaxLimit = pSET2.Upper_Angle_Limit;
								mid2.AngleMinLimit = pSET2.Lower_Angle_Limit;
								mid2.AngleFinalTarget = pSET2.Angle_Target;
								Random random3 = new Random();
								decimal num8 = (decimal)random3.Next(-10, 9) * 0.9m;
								mid2.Angle = decimal.ToInt32(num8 + (decimal)pSET2.Angle_Target);
								mid2.AngleStatus = TighteningValueStatus.OK;
								mid2.TighteningStatus = true;
								num8 = (decimal)random3.Next(-100000000, 900000000) * 0.00000000001m;
								decimal torque2 = pSET2.Torque_Target + pSET2.Torque_Target * num8;
								mid2.Torque = torque2;
								mid2.TorqueFinalTarget = pSET2.Torque_Target;
								mid2.TorqueMaxLimit = pSET2.Upper_Torque_Limit;
								mid2.TorqueMinLimit = pSET2.Lower_Torque_Limit;
								mid2.TorqueControllerName = Name;
								mid2.TorqueStatus = TighteningValueStatus.OK;
								mid2.VinNumber = VIN;
								mid2.BatchCounter = 0;
								mid2.StrategyOptions = new StrategyOptions
								{
									Torque = true,
									Angle = true
								};
								TighteningErrorStatus tighteningErrorStatus3 = new TighteningErrorStatus();
								TighteningErrorStatus2 tighteningErrorStatus4 = new TighteningErrorStatus2();
								mid2.TighteningErrorStatus = tighteningErrorStatus3;
								mid2.TighteningErrorStatus2 = tighteningErrorStatus4;
								mid2.CellId = 0;
								mid2.ChannelId = 0;
								mid2.JobId = 0;
								mid2.ParameterSetId = Selected_PSET_ID;
								mid2.ParameterSetName = pSET2.Name;
								mid2.BatchSize = 1;
								mid2.BatchCounter = 1;
								mid2.BatchStatus = BatchStatus.OK;
								mid2.Timestamp = DateTime.Now;
								mid2.LastChangeInParameterSet = DateTime.Now;
								mid2.TighteningId = num3++;
								byte[] array2 = mid2.PackBytesWithNul();
								try
								{
									stream.Write(array2, 0, array2.Length);
									stream.FlushAsync();
								}
								catch (Exception)
								{
								}
								bool fail = true;
								SendMID1202(fail, stream);
								break;
							}
							case 'F':
							{
								Program.Log(Name + " Send Result NOK");
								PSET pSET = PSETs.Find((PSET e) => e.ID == Selected_PSET_ID);
								Mid0061 mid = new Mid0061(1);
								mid.AngleMaxLimit = pSET.Upper_Angle_Limit;
								mid.AngleMinLimit = pSET.Lower_Angle_Limit;
								mid.AngleFinalTarget = pSET.Angle_Target;
								Random random2 = new Random();
								mid.Angle = (int)((double)pSET.Angle_Target * 0.01) + pSET.Angle_Target;
								mid.AngleStatus = TighteningValueStatus.LOW;
								mid.TighteningStatus = false;
								decimal num7 = (decimal)random2.Next(-100000000, 900000000) * 0.000000001m;
								decimal torque = pSET.Torque_Target + pSET.Torque_Target * num7;
								mid.Torque = torque;
								mid.TorqueFinalTarget = pSET.Torque_Target;
								mid.TorqueMaxLimit = pSET.Upper_Torque_Limit;
								mid.TorqueMinLimit = pSET.Lower_Torque_Limit;
								mid.TorqueControllerName = Name;
								mid.TorqueStatus = TighteningValueStatus.HIGH;
								mid.VinNumber = VIN;
								mid.BatchCounter = 0;
								mid.StrategyOptions = new StrategyOptions
								{
									Torque = true,
									Angle = true
								};
								TighteningErrorStatus tighteningErrorStatus = new TighteningErrorStatus();
								TighteningErrorStatus2 tighteningErrorStatus2 = new TighteningErrorStatus2();
								mid.TighteningErrorStatus = tighteningErrorStatus;
								mid.TighteningErrorStatus2 = tighteningErrorStatus2;
								mid.CellId = 0;
								mid.ChannelId = 0;
								mid.JobId = 0;
								mid.ParameterSetId = Selected_PSET_ID;
								mid.ParameterSetName = pSET.Name;
								mid.BatchSize = 1;
								mid.BatchCounter = 1;
								mid.BatchStatus = BatchStatus.OK;
								mid.Timestamp = DateTime.Now;
								mid.LastChangeInParameterSet = DateTime.Now;
								mid.TighteningId = num3++;
								byte[] array = mid.PackBytesWithNul();
								try
								{
									stream.Write(array, 0, array.Length);
									stream.FlushAsync();
								}
								catch (Exception)
								{
								}
								break;
							}
							}
							num = ((num != num2 - 1) ? (num + 1) : 0);
							Task.Delay(ResultDelay).Wait();
						}
						else
						{
							Program.Log(Name + " in Rundowns() TOOL_ENABLED false");
						}
					}
					else
					{
						Program.Log(Name + " in Rundowns() PSET_Selected_By_Integrator false");
					}
					Task.Delay(2000).Wait();
				}
                Task.Delay(200).Wait();
            }
		});
	}
}
