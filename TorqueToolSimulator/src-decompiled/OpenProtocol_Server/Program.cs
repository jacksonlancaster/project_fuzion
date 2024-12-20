using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Reflection;
using System.Threading.Tasks;

namespace OpenProtocol_Server;

internal class Program
{
	public static Tool core;

	public static bool closing;

	public static Queue<string> logQueue;

	private static async Task Main(string[] args)
	{
		AppDomain.CurrentDomain.UnhandledException += CurrentDomain_UnhandledException;
		Assembly assembly = Assembly.GetExecutingAssembly();
		FileVersionInfo fvi = FileVersionInfo.GetVersionInfo(assembly.Location);
		string version = fvi.FileVersion;
		Console.CancelKeyPress += Console_CancelKeyPress;
		Console.WriteLine("Open Protocol Server " + version);
		logQueue = new Queue<string>();
		Log("Open Protocol Server startup");
		Core core = new Core();
		await core.Start();

		Console.ReadLine();
	}

	private static void CurrentDomain_UnhandledException(object sender, UnhandledExceptionEventArgs e)
	{
		Console.WriteLine((e.ExceptionObject as Exception).Message);
	}

	private static void Console_CancelKeyPress(object? sender, ConsoleCancelEventArgs e)
	{
		Log("Ctl-C pressed. Closing Application");
		closing = true;
		core.Dispose();
	}

	public static void Log(string v, ConsoleColor c = ConsoleColor.Black)
	{
		string time = DateTime.Now.ToString("o");
		Console.BackgroundColor = c;
		Console.WriteLine(time + ": " + v);
		Console.ResetColor();
		logQueue.Enqueue(time + ": " + v + "\n");
	}

	public static void Log(string value)
	{
		Log(value, ConsoleColor.Black);
	}

	public static void Log(Exception ex)
	{
		string InnerException = "";
		if (ex.InnerException != null)
		{
			InnerException = ex.InnerException.Message;
		}
		string StackTrace = ex.StackTrace;
		Log($"{ex.Message} {InnerException} {StackTrace}", ConsoleColor.Red);
	}

	public static void Log(string err, Exception ex)
	{
		string InnerException = "";
		try
		{
			InnerException = ex.InnerException.Message;
		}
		catch
		{
		}
		string StackTrace = ex.StackTrace;
		Log($"{err} {ex.Message} {InnerException} {StackTrace}", ConsoleColor.Red);
	}

	public static bool IsFileReady(string sFilename)
	{
		try
		{
			using FileStream inputStream = File.Open(sFilename, FileMode.Open, FileAccess.Read, FileShare.None);
			return inputStream.Length > 0;
		}
		catch (FileNotFoundException)
		{
			File.Create(sFilename);
			return false;
		}
		catch (Exception)
		{
			return false;
		}
	}
}
