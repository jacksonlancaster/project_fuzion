using System.Collections.Generic;
using System.Linq;
using Newtonsoft.Json;

namespace OpenProtocol_Server;

public class ToolConfig
{
	public int[] Tool_PSETS;

	[JsonIgnore]
	public List<PSET> PSETs = new List<PSET>();

	public string Name { get; set; }

	public string Port { get; set; }

	public string ResultPattern { get; set; }

	public int ResultDelay { get; set; }

	public void InitToolConfig(List<PSET> PSETs)
	{
		int[] tool_PSETS = Tool_PSETS;
		foreach (int psetNumber in tool_PSETS)
		{
			if (psetNumber == 0)
			{
				this.PSETs = PSETs.ToList();
				break;
			}
			if (psetNumber != 0)
			{
				PSET tmp = PSETs.Find((PSET x) => x.ID == psetNumber);
				this.PSETs.Add(tmp);
			}
		}
	}
}
