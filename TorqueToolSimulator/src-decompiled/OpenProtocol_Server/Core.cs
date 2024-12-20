using System;
using System.Collections.Generic;
using System.IO;
using System.Threading.Tasks;
using Newtonsoft.Json;

namespace OpenProtocol_Server;

public class Core : IDisposable
{
	public List<Tool> Tools = new List<Tool>();

	private Config config;

	public bool disposed;

	public Core()
	{
		InitConfig();
	}

	public void Dispose()
	{
	}

	public async Task Start()
	{
		Console.WriteLine("Core.Start()");
		foreach (ToolConfig tconfig in config.Tool)
		{
			tconfig.InitToolConfig(config.PSETs);
			Console.WriteLine("Core.Start() creating new tool " + tconfig.Name);
			Tool tool = new Tool(tconfig.Name, tconfig.Port, tconfig.ResultPattern, tconfig.ResultDelay, tconfig.PSETs);
			tool.Start();
			Tools.Add(tool);
		}
		//while (!disposed)
		//{
		//	Task.Delay(2000).Wait();
		//}
	}

	private void InitConfig()
	{
		config = new Config();
		string JSONConfig = "";
		try
		{
			if (!File.Exists(".\\Config.json"))
			{
				Program.Log("in InitConfig() \\Config.json file does not exist");
				Program.Log("in InitConfig() Exiting Program");
				Environment.Exit(-1);
			}
			Console.WriteLine("in InitConfig() read json config");
			JSONConfig = File.ReadAllText(".\\Config.json");
		}
		catch (Exception ex)
		{
			Program.Log("in InitConfig()", ex);
		}
		try
		{
			config = JsonConvert.DeserializeObject<Config>(JSONConfig);
		}
		catch (Exception ex1)
		{
			Program.Log("in InitConfig", ex1);
		}
	}
}
