namespace OpenProtocol_Server;

public class PSET
{
	public int ID { get; set; }

	public decimal Upper_Torque_Limit { get; set; }

	public decimal Lower_Torque_Limit { get; set; }

	public decimal Torque_Target { get; set; }

	public int Upper_Angle_Limit { get; set; }

	public int Lower_Angle_Limit { get; set; }

	public int Angle_Target { get; set; }

	public string Name { get; set; }
}
