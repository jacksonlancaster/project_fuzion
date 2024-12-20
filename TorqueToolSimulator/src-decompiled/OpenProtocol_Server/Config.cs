using System.Collections.Generic;

namespace OpenProtocol_Server;

public class Config
{
	public List<ToolConfig> Tool;

	public List<PSET> PSETs { get; set; }
}
