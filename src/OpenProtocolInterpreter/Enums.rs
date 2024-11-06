
 /// <summary>
/// Angle status. Used in <see cref="PowerMACS.BoltData"/>.
/// </summary>
#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum AngleStatus
{
    Undefined = -1,
    BoltAngleLow = 0,
    BoltAngleOk = 1,
    BoltAngleHigh = 2
}

/// <summary>
/// Auto Select. Used in <see cref="Job.Advanced.Mid0140"/> in <see cref="Job.Advanced.AdvancedJob"/> .
/// </summary>
pub enum AutoSelect
{
    None = 0,
    AutoNextChange = 1,
    IO = 2,
    Fieldbus = 6,
    SocketTray = 8,
    EndFittings = 9,
    ToolDisplay = 10
}

/// <summary>
/// Batch Mode. Used in <see cref="Job.Advanced.Mid0140"/>
/// </summary>
pub enum BatchMode
{
    OnlyOk = 0,
    BothOkAndNok = 1
}

/// <summary>
/// Batch Status. Used in <see cref="Tightening.Mid0061"/> and <see cref="Tightening.Mid0065"/>.
/// </summary>
pub enum BatchStatus
{
    Nok = 0,
    Ok = 1,
    NotUsed = 2,
    Running = 3
}

/// /// <summary>
/// Batch Status at Increment. Used in <see cref="Job.Advanced.Mid0140"/> in <see cref="Job.Advanced.AdvancedJob"/> .
/// </summary>
pub enum BatchStatusAtIncrement
{
    Ok = 0,
    Nok = 1
}

/// <summary>
/// Calibration Units. Used in <see cref="Tool.Mid0045"/>.
/// </summary>
pub enum CalibrationUnit
{
    Nm = 1,
    LbfFt = 2,
    LbfIn = 3,
    Kpm = 4,
    KgfCm = 5,
    OzfIn = 6,
    Percentage = 7,
    Ncm = 8
}

/// <summary>
/// Current Batch Status. Used in <see cref="Job.Advanced.Mid0140"/> in <see cref="Job.Advanced.AdvancedJob"/>.
/// </summary>
pub enum CurrentBatchStatus
{
    NotStarted = 0,
    Ok = 1,
    Nok = 2
}

pub enum DataTypeDefinition
{
    /// <summary>
    /// <c>UI</c> - The value is an unsigned integer. The number of digits are defined with the Length parameter
    /// </summary>
    UnsignedInteger = 1,
    /// <summary>
    /// <c>I</c> - The value is a signed integer. The number of digits are defined with the Length parameter
    /// </summary>
    Integer = 2,
    /// <summary>
    /// <para><c>F</c> - The value is sent as a float value with the layout <c>12.12</c>, <c>10025.1234</c> or <c>-57.5</c>.</para>
    /// <para>It is up to the sender of the telegram to decide the number of decimals to send.</para>
    /// The number of characters sent varies depending on the size and resolution of the sent number.
    /// </summary>
    Float = 3,
    /// <summary>
    /// <para><c>S</c> - The value is a string. Sent as ASCII characters, the length of the data fits the actual length of the string. </para>
    /// <em>Note that the string may contain spaces (ASCII character 0x20)</em> 
    /// </summary>
    String = 4,
    /// <summary>
    /// <c>S</c> - A time specified by 19 ASCII characters (<c>YYYY-MM-DD:HH:MM:SS</c>)
    /// </summary>
    Timestamp = 5,
    /// <summary>
    /// <para><c>B</c> - A boolean value, one ASCII digit:</para>
    /// <example>0 = FALSE | 1 = TRUE</example>
    /// </summary>
    Boolean = 6,
    /// <summary>
    /// <c>H</c> - Hexadecimal value. Sent as ASCII characters, example: <c>A24CD3</c>.
    /// </summary>
    Hexadecimal = 7,
    /// <summary>
    /// <c>PL1</c> - Plotting point consisting of a FA of one pair of float values where the first value is the Y and the second is the X within the pair.
    /// </summary>
    PlottingPoint1 = 8,
    /// <summary>
    /// <c>PL2</c> - Plotting point consisting of a FA of two pairs of float values where the first value is the Y and the second is the X within a pair.
    /// </summary>
    PlottingPoint2 = 9,
    /// <summary>
    /// <c>PL4</c> - Plotting point consisting of a FA of 4 pairs of float values where the first value is the Y and the second is the X within a pair.
    /// </summary>
    PlottingPoint4 = 10,
    /// <summary>
    /// <para><c>FA</c> - Array of Float. Each float value is sent as 8 ASCII characters.</para> 
    /// <para>Negative values start with a ‘-‘ sign. The precision of the values vary, for large values decimal point is omitted.</para>
    /// <para>Valid values are for example <c>-1234567</c>, <c>001.1205</c>, <c>-123.789</c></para>
    /// </summary>
    FloatArray = 50,
    /// <summary>
    /// <para><c>UA</c> - Array of Unsigned integers. Each integer value is sent as 8 ASCII characters.</para> 
    /// <para>Valid values are for example <c>12345678</c>, <c>00001234</c>, <c>00200000</c></para>
    /// </summary>
    UnsignedIntegerArray = 51,
    /// <summary>
    /// <para><c>IA</c> - Array of Signed integers. Each integer value is sent as 8 ASCII characters. Negative values start with a ‘-‘ sign.</para> 
    /// <para>Valid values are for example <c>12345678</c>, <c>-1234567</c>, <c>00200000</c>, <c>10200000</c></para>
    /// </summary>
    IntegerArray = 52
}

pub enum DataUnitType
{
    NoUnit = 0,

    //Torque units
    /// <summary>
    /// Newton meter
    /// </summary>
    Nm = 1,
    /// <summary>
    /// Foot-pound force
    /// </summary>
    FtLbf = 2,
    /// <summary>
    /// Centi Newton meter
    /// </summary>
    CNm = 3,
    /// <summary>
    /// Kilo Newton meter
    /// </summary>
    KNm = 4,
    /// <summary>
    /// Mega Newton meter
    /// </summary>
    MNm = 5,
    /// <summary>
    /// Inch-pound force
    /// </summary>
    InLbf = 6,
    /// <summary>
    /// Kilo pound meter
    /// </summary>
    Kpm = 7,
    /// <summary>
    /// Kilo centi force
    /// </summary>
    
    #[deprecated]
    Kfcnm = 8,

    #[deprecated]
    TorquePercentage = 9,
    OzfIn = 10,
    DNm = 11,
    /// <summary>
    /// Milli-newton meter
    /// </summary>
    MiNm = 12,
    /// <summary>
    /// Kilogram force centimeter
    /// </summary>
    KgfCm = 13,
    /// <summary>
    /// Gram force centimeter
    /// </summary>
    GfCm = 14,
    /// <summary>
    /// Ounce force foot
    /// </summary>
    FtOzf = 15,

    //Angle units
    Degree = 50,
    Radian = 51,

    //Frequency units
    /// <summary>
    /// Hertz
    /// </summary>
    Hz = 100,
    /// <summary>
    /// Revolutions per minute
    /// </summary>
    Rpm = 101,

    //Torque rate units
    /// <summary>
    /// Newton meter / degree
    /// </summary>
    NmByDegree = 150,
    /// <summary>
    /// Foot-pound force / degree
    /// </summary>
    FtLbfByDegree = 151,
    /// <summary>
    /// Centi Newton meter / degree
    /// </summary>
    CNmByDegree = 152,
    /// <summary>
    /// Kilo Newton meter / degree
    /// </summary>
    KNmByDegree = 153,
    /// <summary>
    /// Mega Newton meter / degree
    /// </summary>
    MNmByDegree = 154,
    /// <summary>
    /// Inch-pound force / degree
    /// </summary>
    InLbfByDegree = 155,

    /// <summary>
    /// Newton meter / radian
    /// </summary>
    NmByRadian = 160,
    /// <summary>
    /// Foot-pound force / radian
    /// </summary>
    FtLbfByRadian = 161,
    /// <summary>
    /// Centi Newton meter / radian
    /// </summary>
    CNmByRadian = 162,

    //Time units
    Second = 200,
    Minute = 201,
    Milliseconds = 202,
    Hour = 203,

    //Temperature units
    Kelvin = 250,
    Celsius = 251,
    Fahrenheit = 252,

    //Force units
    /// <summary>
    /// Newton
    /// </summary>
    N = 300,
    /// <summary>
    /// Kilo newton
    /// </summary>
    KN = 301,
    /// <summary>
    /// Pound-force
    /// </summary>
    Lbf = 302,
    /// <summary>
    /// Kilogram-force
    /// </summary>
    Kgf = 303,
    /// <summary>
    /// Ounce-force
    /// </summary>
    Ozf = 304,
    /// <summary>
    /// Mega newton
    /// </summary>
    MN = 305,

    //Length units
    Meter = 350,
    Millimeter = 351,
    Inch = 352,

    //Speed units
    MeterPerSecond = 400,
    MillimeterPerSecond = 401,

    //Force rate units
    /// <summary>
    /// Newton / millimeter
    /// </summary>
    NByMm = 450,
    /// <summary>
    /// Kilo newton / millimeter
    /// </summary>
    KNByMm = 451,
    /// <summary>
    /// Pound-force / inch
    /// </summary>
    LbfByIn = 452,
    /// <summary>
    /// Kilogram-force / millimeter
    /// </summary>
    KgfByMm = 453,
    /// <summary>
    /// Ounce-force / inch
    /// </summary>
    OzfByIn = 454,
    /// <summary>
    /// Mega newton / millimeter
    /// </summary>
    MNByMm = 455,

    //Acceleration units
    MeterPerSecondSquared = 500,
    MillimeterPerSecondSquared = 501,

    //Mass units
    /// <summary>
    /// Kilogram
    /// </summary>
    Kg = 550,
    /// <summary>
    /// Pound
    /// </summary>
    Lb = 551,

    //Volume units
    Liter = 600,
    CubicMeter = 601,

    //Area units
    SquareMeter = 650,

    //Power units
    Watt = 700,

    //Electric units
    Ampere = 750,
    Volt = 751,
    Ohm = 752,
    Farad = 753,
    Henry = 754,

    //Other units
    Percentage = 800,

    //Pressure units
    /// <summary>
    /// Kilo pascal
    /// </summary>
    KPa = 850,

    //Plotting units
    /// <summary>
    /// Y = Newton meter, X = milliseconds
    /// </summary>
    NmByMilliseconds = 900,
    /// <summary>
    /// Y = foot-pound force, X = milliseconds
    /// </summary>
    FtLbfByMilliseconds = 901,
    /// <summary>
    /// Y = centi Newton meter, X = milliseconds
    /// </summary>
    CNmByMilliseconds = 902,
    /// <summary>
    /// Y = kilo Newton meter, X = milliseconds
    /// </summary>
    KNmByMilliseconds = 903,
    /// <summary>
    /// Y = mega Newton meter, X = milliseconds
    /// </summary>
    MNmByMilliseconds = 904,
    /// <summary>
    /// Y = inch-pound force, X = milliseconds
    /// </summary>
    InLbfByMilliseconds = 905,

    /// <summary>
    /// Y = Degree, X = milliseconds
    /// </summary>
    DegreeByMilliseconds = 910,
    /// <summary>
    /// Y = Radian, X = milliseconds
    /// </summary>
    RadianByMilliseconds = 911,

    /// <summary>
    /// Y = newton, X = milliseconds
    /// </summary>
    NByMilliseconds = 920,
    /// <summary>
    /// Y = kilo newton, X = milliseconds
    /// </summary>
    KNByMilliseconds = 921,
    /// <summary>
    /// Y = pound-force, X = milliseconds
    /// </summary>
    LbfByMilliseconds = 922,
    /// <summary>
    /// Y = kilogram-force, X = milliseconds
    /// </summary>
    KgfByMilliseconds = 923,
    /// <summary>
    /// Y = ounce-force, X = milliseconds
    /// </summary>
    OzfByMilliseconds = 924,
    /// <summary>
    /// Y = mega newton, X = milliseconds
    /// </summary>
    MNByMilliseconds = 925

}

/// /// <summary>
/// Decrement batch after loosening/OK tightening. Used in <see cref="Job.Advanced.Mid0140"/> in <see cref="Job.Advanced.AdvancedJob"/> .
/// </summary>
pub enum DecrementBatchAfterLoosening
{
    Never = 0,
    Always = 1,
    AfterOk = 2
}

/// <summary>
/// Digital Input Numbers. Used in <see cref="IOInterface.Mid0220"/>, <see cref="IOInterface.Mid0221"/>, 
/// <see cref="IOInterface.Mid0223"/>, <see cref="IOInterface.Mid0224"/> and <see cref="IOInterface.Mid0225"/>.
/// </summary>
pub enum DigitalInputNumber
{
    Off = 0,
    ResetBatch = 1,
    UnlockTool = 2,
    ToolDisableNo = 3,
    ToolDisableNc = 4,
    ToolTighteningDisable = 5,
    ToolLooseningDisable = 6,
    RemoteStartPulse = 7,
    RemoteStartCont = 8,
    ToolStartLoosening = 9,
    BatchIncrement = 10,
    BypassPSet = 11,
    AbortJob = 12,
    JobOff = 13,
    ParameterSetToggle = 14,
    ResetRelays = 15,
    ParameterSetSelectBit0 = 16,
    ParameterSetSelectBit1 = 17,
    ParameterSetSelectBit2 = 18,
    ParameterSetSelectBit3 = 19,
    JobSelectBit0 = 20,
    JobSelectBit1 = 21,
    JobSelectBit2 = 22,
    JobSelectBit3 = 23,

    LineControlStart = 28,
    LineControlAlert1 = 29,
    LineControlAlert2 = 30,
    AckErrorMessage = 31,
    FieldbusDigIn1 = 32,
    FieldbusDigIn2 = 33,
    FieldbusDigIn3 = 34,
    FieldbusDigIn4 = 35,
    FlashToolGreenLight = 36,

    ManualMode = 43,
    ParameterSetSelectBit4 = 45,
    ParameterSetSelectBit5 = 46,
    ParameterSetSelectBit6 = 47,
    ParameterSetSelectBit7 = 48,
    JobSelectBit4 = 49,
    JobSelectBit5 = 50,
    JobSelectBit6 = 51,
    JobSelectBit7 = 52,
    BatchDecrement = 53,
    JobRestart = 54,
    EndOfCycle = 55,

    ClickWrench1 = 62,
    ClickWrench2 = 63,
    ClickWrench3 = 64,
    ClickWrench4 = 65,
    IdCard = 66,
    AutomaticMode = 67,
    ExternalMonitored1 = 68,
    ExternalMonitored2 = 69,
    ExternalMonitored3 = 70,
    ExternalMonitored4 = 71,
    ExternalMonitored5 = 72,
    ExternalMonitored6 = 73,
    ExternalMonitored7 = 74,
    ExternalMonitored8 = 75,
    SelectNextParameterSet = 76,
    SelectPreviousParameterSet = 77,

    TimerEnableTool = 79,
    MasterUnlockTool = 80,
    STScanRequest = 81,
    DisconnectTool = 82,
    JobSelectBit8 = 83,
    ParameterSetSelectBit8 = 84,
    RequestSTScan = 85,
    ResetNokCounter = 86,
    BypassIdentifier = 87,
    ResetLatestIdentifier = 88,
    ResetAllIdentifier = 89,
    SetHomePosition = 90,
    DigoutMonitored1 = 91,
    DigoutMonitored2 = 92,
    DigoutMonitored3 = 93,
    DigoutMonitored4 = 94,
    DisableSTScanner = 95,
    DisableFieldbusCarriedSignals = 96,
    ToggleCwCcw = 97,
    ToggleCwCcwForNextRun = 98,
    SetCcw = 99,

    OpenProtocolCommandsDisable = 104,
    LogicDigIn1 = 105,
    LogicDigIn2 = 106,
    LogicDigIn3 = 107,
    LogicDigIn4 = 108,
    LogicDigIn5 = 109,
    LogicDigIn6 = 110,
    LogicDigIn7 = 111,
    LogicDigIn8 = 112,
    LogicDigIn9 = 113,
    LogicDigIn10 = 114,

    ForcedCcwOnce = 120,
    ForcedCcwToggle = 121,
    ForcedCwOnce = 122,
    ForcedCwToggle = 123,

    ParameterSetSelectBit9 = 129,
    StoreCurrentTighteningProgramInTheTool = 130,
    ActiveXmlResultSend = 131,
    ToolInWorkSpace = 132,
    ToolInProductSpace = 133,
    FlashToolYellowLight = 134,
    XmlEmergencyMode = 135,
    MFUTest = 136,
    ToolInParkPosition = 137,
    EnableOperation = 138,
    StopTightening = 139,
    StartLooseningPulse = 140,

    PulsorToolEnable = 150,
    PerformAirHoseTest = 151,
    LastDigIn = 152,

    ToolBlueLightIOControlled = 201,
    ToolBlueLight = 202,
    ToolGreenLightIOControlled = 203,
    ToolGreenLight = 204,
    ToolRedLightIOControlled = 205,
    ToolRedLight = 206,
    ToolYellowLightIOControlled = 207,
    ToolYellowLight = 208,
    ToolWhiteLightIOControlled = 209,
    ToolWhiteLight = 210
}

/// <summary>
/// Disable types. Used in <see cref="Tool.Mid0042"/>.
/// </summary>
pub enum DisableType
{
    /// <summary>
    /// This is the same function as the revision 1 functionality. The tool is locked and cannot be started.
    /// </summary>
    Disable = 0,
    /// <summary>
    /// Will not run in the next tightening but will be included in the final result with status NOK
    /// </summary>
    InhibitNok = 1,
    /// <summary>
    /// Will not run in the next tightening but will be included in the final result with status OK
    /// </summary>
    InhibitOk = 2,
    /// <summary>
    /// Will not run in the next tightening and will not be included in the final result
    /// </summary>
    InhibitNoResult = 3
}

/// <summary>
/// <see cref="Communication.Mid0004"/> possible errors.
/// </summary>
pub enum Error
{
    NoError = 00,
    InvalidData = 01,
    ParameterSetIdNotPresent = 02,
    ParameterSetCannotBeSet = 03,
    ParameterSetNotRunning = 04,
    VINUploadSubscriptionAlreadyExists = 06,
    VINUploadSubscriptionDoesntExists = 07,
    VINInputSourceNotGranted = 08,
    LastTighteningResultSubscriptionAlreadyExists = 09,
    LastTighteningResultSubscriptionDoesntExists = 10,
    AlarmSubscriptionAlreadyExists = 11,
    AlarmSubscriptionDoesntExists = 12,
    ParameterSetSelectionSubscriptionAlreadyExists = 13,
    ParameterSetSelectionSubscriptionDoesntExists = 14,
    TighteningIdRequestNotFound = 15,
    ConnectionRejectedProtocolBusy = 16,
    JobIdNotPresent = 17,
    JobInfoSubscriptionAlreadyExists = 18,
    JobInfoSubscriptionDoesntExists = 19,
    JobCannotBeSet = 20,
    JobNotRunning = 21,
    NotPossibleToExecuteDynamicJobRequest = 22,
    JobBatchDecrementFailed = 23,
    NotPossibleToCreatePSet = 24,
    ProgrammingControlNotGranted = 25,
    WrongToolTypeToPSetDownloadConnected = 26,
    ToolIsInaccessible = 27,
    JobAbortionIsInProgress = 28,
    ToolDoesNotExist = 29,
    ControllerIsNotASyncMasterOrStationController = 30,
    MultiSpindleStatusSubscriptionAlreadyExists = 31,
    MultiSpindleStatusSubscriptionDoesntExists = 32,
    MultiSpindleResultSubscriptionAlreadyExists = 33,
    MultiSpindleResultSubscriptionDoesntExists = 34,
    OtherMasterClientAlreadyConnected = 35,
    LockTypeNotSupported = 36,
    JobLineControlInfoSubscriptionAlreadyExists = 40,
    JobLineControlInfoSubscriptionDoesntExists = 41,
    IdentifierInputSourceNotGranted = 42,
    MultipleIdentifiersWorkOrderSubscriptionAlreadyExists = 43,
    MultipleIdentifiersWorkOrderSubscriptionDoesntExists = 44,
    StatusExternalMonitoredInputsSubscriptionAlreadyExists = 50,
    StatusExternalMonitoredInputsSubscriptionDoesntExists = 51,
    IODeviceNotConnected = 52,
    FaultyIODeviceId = 53,
    ToolTagIdUnknown = 54,
    ToolTagIdSubscriptionAlreadyExists = 55,
    ToolTagIdSubscriptionDoesntExists = 56,
    ToolMotorTuningFailed = 57,
    NoAlarmPresent = 58,
    ToolCurrentlyInUse = 59,
    NoHistogramAvailable = 60,
    PairingFailed = 61,
    PairingDenied = 62,
    PairingOrPairingAbortionAttemptOnWrongToolType = 63,
    PairingAbortionDenied = 64,
    PairingAbortionFailed = 65,
    PairingDisconnectionFailed = 66,
    PairingInProgressOrAlreadyDone = 67,
    PairingDeniedNoProgramControl = 68,
    UnsupportedExtraDataRevision = 69,
    CalibrationFailed = 70,
    SubscriptionAlreadyExists = 71,
    SubscriptionDoesntExists = 72,
    SubscribedMidUnsupported = 73,
    SubscribedMidRevisionUnsupported = 74,
    RequestedMidUnsupported = 75,
    RequestedMidRevisionUnsupported = 76,
    RequestedOnSpecificDataNotSupported = 77,
    SubscriptionOnSpecificDataNotSupported = 78,
    CommandFailed = 79,
    AudiEmergencyStatusSubscriptionExists = 80,
    AudiEmergencyStatusSubscriptionDoesntExists = 81,
    AutomaticManualModeSubscribeAlreadyExists = 82,
    AutomaticManualModeSubscribeDoesntExists = 83,
    RelayFunctionSubscriptionAlreadyExists = 84,
    RelayFunctionSubscriptionDoesntExists = 85,
    SelectorSocketInfoSubscriptionAlreadyExists = 86,
    SelectorSocketInfoSubscriptionDoesntExists = 87,
    DigInInfoSubscriptionAlreadyExists = 88,
    DigInInfoSubscriptionDoesntExists = 89,
    LockAtBatchDoneSubscriptionAlreadyExists = 90,
    LockAtBatchDoneSubscriptionDoesntExists = 91,
    OpenProtocolCommandsDisabled = 92,
    OpenProtocolCommandsDisabledSubscriptionAlreadyExists = 93,
    OpenProtocolCommandsDisabledSubscriptionDoesntExists = 94,
    RejectRequestPowerMACSIsInManualMode = 95,
    ClientAlreadyConnected = 96,
    MidRevisionUnsupported = 97,
    ControllerInternalRequestTimeout = 98,
    UnknownMid = 99,
    IllegalPID = 100,
    TighteningInProgress = 101,
    DeleteOfObjectNotPossible = 102,
    IllegalProgramId = 103,
    IllegalNodeType = 104
}

/// <summary>
/// Forced Orders. Used in <see cref="Job.Mid0033"/> and <see cref="Job.Advanced.Mid0140"/>.
/// </summary>
pub enum ForcedOrder
{
    FreeOrder = 0,
    ForcedOrder = 1,
    FreeAndForcedOrder = 2
}

/// <summary>
/// Histogram Types. Used in <see cref="Statistic.Mid0300"/> and <see cref="Statistic.Mid0301"/>.
/// </summary>
pub enum HistogramType
{
    Torque = 0,
    Angle = 1,
    Current = 2,
    PrevailTorque = 3,
    Selftap = 4,
    RundownAngle = 5
}

/// <summary>
/// Identifier parts. Used in <see cref="Job.Advanced.Mid0140"/>.
/// </summary>
pub enum IdentifierPart
{
    JobVINNumber = 0,
    Other = 1
}

/// <summary>
/// Mid interpreter possible modes.
/// </summary>
pub enum InterpreterMode
{
    Both = 0,
    Controller = 1,
    Integrator = 2
}

/// <summary>
/// Job batch modes. Used in <see cref="Job.Mid0033"/> and <see cref="Job.Mid0035"/>.
/// </summary>
pub enum JobBatchMode
{
    OnlyOkTightenings = 0,
    OkAndNokTightenings = 1
}

/// <summary>
/// Job Status. Used in <see cref="Job.Mid0035"/>.
/// </summary>
pub enum JobStatus
{
    NotCompleted = 0,
    Ok = 1,
    Nok = 2
}

/// <summary>
/// Job tightening status. Used in <see cref="Job.Mid0035"/>.
/// </summary>
pub enum JobTighteningStatus
{
    Off = 0,
    Nok = 1,
    Aborted = 3,
    Incremented = 4,
    Decremented = 5,
    Bypassed = 6,
    ResetBatch = 7,
    Loosening = 8,
    FreeBatch = 9,
    JobAborted = 10,
    JobRestart = 11
}

/// <summary>
/// Light commands. Used in <see cref="ApplicationSelector.Mid0254"/> and <see cref="ApplicationSelector.Mid0255"/>.
/// </summary>
pub enum LightCommand
{
    Off = 0,
    Steady = 1,
    Flashing = 2
}

/// <summary>
/// Error codes. Used in <see cref="LinkCommunication.Mid9998"/>.
/// </summary>
pub enum LinkCommunicationError
{
    InvalidLength = 1,
    InvalidRevision = 2,
    InvalidSequenceNumber = 3,
    InconsistencyOfMessageNumber = 4
}

/// <summary>
/// Motor rotations. Used in <see cref="Tool.Mid0041"/>.
/// </summary>
pub enum MotorRotation
{
    Normal = 0,
    Inverted = 1
}

pub enum NodeType
{
    ParameterSet = 1,
    Multistage = 2,
    Job = 3,

    TighteningProgram = 100,
    TighteningStep = 101,
    Restriction = 102,
    Check = 103,
    SpeedRamp = 104,
    Monitoring = 105,

    MultistepTighteningProgram = 201
}

pub enum ObjectType
{
    Unknown = 0,
    DualReading = 1,
    TighteningProduction = 2,
    TighteningSimulation = 3,
    JointCheck = 4,
    Dimensional = 5
}

/// <summary>
/// Operation Types. Used in <see cref="Result.Mid1201"/>.
/// </summary>
pub enum OperationType
{
    NonSynchronizedTightening = 0,
    SynchronizedTightening = 1,
    Pressing = 2,
    Drilling = 3,
    Pulse = 4
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq)]
pub enum PaddingOrientation
{
    #[default] RightPadded,
    LeftPadded
}

/// <summary>
/// Pairing handling types. Used in <see cref="Tool.Mid0047"/>.
/// </summary>
pub enum PairingHandlingType
{
    StartPairing = 01,
    PairingAbortOrDisconnect = 02,
    FetchLatestPairingStatus = 03
}

/// <summary>
/// Pairing status. Used in <see cref="Tool.Mid0048"/>.
/// </summary>
pub enum PairingStatus
{
    /// <summary>
    /// Tool not mounted yet
    /// </summary>
    Undefined = 0,
    /// <summary>
    /// Pairing allowed and started
    /// </summary>
    Accepted = 1,
    /// <summary>
    /// Normal pairing sequence as OK
    /// </summary>
    Inquiry = 2,
    /// <summary>
    /// Normal pairing sequence as OK
    /// </summary>
    SendPin = 3,
    /// <summary>
    /// Normal pairing sequence as OK
    /// </summary>
    PinOk = 4,
    /// <summary>
    /// Normal pairing sequence as OK
    /// </summary>
    Ready = 5,
    /// <summary>
    /// Ongoing Pairing Aborted
    /// </summary>
    Aborted = 6,
    /// <summary>
    /// Pairing not allowed. Program control.
    /// </summary>
    Denied = 7,
    /// <summary>
    /// Pairing attempt failed
    /// </summary>
    Failed = 8,
    /// <summary>
    /// Pairing never done before or disconnected
    /// </summary>
    Unready = 9
}

/// <summary>
/// Post View Torque. Used in <see cref="Tightening.Mid0065"/>.
/// </summary>
pub enum PostViewTorque
{
    Off = 0,
    On = 1,
    OnlyPVTHOn = 2,
    OnlyPVTLOn = 3
}

/// <summary>
/// Power Macs status. Used in <see cref="PowerMACS.Mid0106"/>.
/// </summary>
pub enum PowerMacsStatus
{
    Ok = 0,
    Okr = 1,
    Nok = 2,
    TermNok = 3
}

/// <summary>
/// Primary tools. Used in <see cref="Tool.Mid0041"/> and <see cref="Tool.Mid0046"/>.
/// </summary>
pub enum PrimaryTool
{
    Cable = 1,
    IRC_B = 2,
    IRC_W = 3
}

/// <summary>
/// Relay Numbers. Used in <see cref="IOInterface.Mid0216"/>, <see cref="IOInterface.Mid0217"/>, <see cref="IOInterface.Mid0219"/> and <see cref="IOInterface.Relay"/>
/// </summary>
pub enum RelayNumber
{
    Off = 0,
    Ok = 1,
    Nok = 2,
    Low = 3,
    High = 4,
    LowTorque = 5,
    HighTorque = 6,
    LowAngle = 7,
    HighAngle = 8,
    CycleComplete = 9,
    Alarm = 10,
    BatchNxOk = 11,
    JobOk = 12,
    JobNok = 13,
    JobRunning = 14,

    ToolHealthOk = 17,
    PowerFocusReady = 18,
    ToolReady = 19,
    ToolStartSwitch = 20,
    DirectionSwitchClockwise = 21,
    DirectionSwitchCounterclockwise = 22,
    TighteningDirectionCounterclockwise = 23,
    ToolTightening = 24,
    ToolLoosening = 25,
    ToolRunning = 26,
    ToolRunningClockwise = 27,
    ToolRunningCounterclockwise = 28,
    StatisticAlarm = 29,
    ToolLocked = 30,
    ReceivedIdentifier = 31,
    RunningPSetBit0 = 32,
    RunningPSetBit1 = 33,
    RunningPSetBit2 = 34,
    RunningPSetBit3 = 35,
    RunningJobBit0 = 36,
    RunningJobBit1 = 37,
    RunningJobBit2 = 38,
    RunningJobBit3 = 39,

    LineOk = 44,
    LineControlAlert1 = 45,
    LineControlAlert2 = 46,
    ServiceIndicator = 47,
    FieldbusRelay1 = 48,
    FieldbusRelay2 = 49,
    FieldbusRelay3= 50,
    FieldbusRelay4 = 51,
    ToolRedLight = 52,
    ToolGreenLight = 53,
    ToolYellowLight = 54,

    RunningPSetBit4 = 59,
    RunningPSetBit5 = 60,
    RunningPSetBit6 = 61,
    RunningPSetBit7 = 62,
    RunningJobBit4 = 63,
    RunningJobBit5 = 64,
    RunningJobBit6 = 65,
    RunningJobBit7 = 66,
    SyncOk = 67,
    SyncNok = 68,
    SyncSpindle1Ok = 69,
    SyncSpindle1Nok = 70,
    SyncSpindle2Ok = 71,
    SyncSpindle2Nok = 72,
    SyncSpindle3Ok = 73,
    SyncSpindle3Nok = 74,
    SyncSpindle4Ok = 75,
    SyncSpindle4Nok = 76,
    SyncSpindle5Ok = 77,
    SyncSpindle5Nok = 78,
    SyncSpindle6Ok = 79,
    SyncSpindle6Nok = 80,
    SyncSpindle7Ok = 81,
    SyncSpindle7Nok = 82,
    SyncSpindle8Ok = 83,
    SyncSpindle8Nok = 84,
    SyncSpindle9Ok = 85,
    SyncSpindle9Nok = 86,
    SyncSpindle10Ok = 87,
    SyncSpindle10Nok = 88,

    LineControlStart = 91,
    JobAborted = 92,
    ExternalControlled1 = 93,
    ExternalControlled2 = 94,
    ExternalControlled3 = 95,
    ExternalControlled4 = 96,
    ExternalControlled5 = 97,
    ExternalControlled6 = 98,
    ExternalControlled7 = 99,
    ExternalControlled8 = 100,
    ExternalControlled9 = 101,
    ExternalControlled10 = 102,
    ToolsNetConnectionLost = 103,
    OpenProtocolConnectionLost = 104,
    FieldbusOffline = 105,
    HomePosition = 106,
    BatchNok = 107,
    SelectedChannelInJob = 108,
    SafeToDisconnectTool = 109,
    RunningJobBit8 = 110,
    RunningPSetBit8 = 111,
    CalibrationAlarm = 112,
    CycleStart = 113,
    LowCurrent = 114,
    HighCurrent = 115,
    LowPVTMonitoring = 116,
    HighPVTMonitoring = 117,
    LowPVTSelftap = 118,
    HighPVTSelftap = 119,
    LowTighteningAngle = 120,
    HighTighteningAngle = 121,
    IdentifierIdentified = 122,
    IdentifierType1Received = 123,
    IdentifierType2Received = 124,
    IdentifierType3Received = 125,
    IdentifierType4Received = 126,

    RingButtonAck = 129,
    DigInControlled1 = 130,
    DigInControlled2 = 131,
    DigInControlled3 = 132,
    DigInControlled4 = 133,
    FieldbusCarriedSignalsDisabled = 134,
    Illuminator = 135,
    NewParameterSetSelected = 136,
    NewJobSelected = 137,
    JobOffRelay = 138,
    LogicRelay1 = 139,
    LogicRelay2 = 140,
    LogicRelay3 = 141,
    LogicRelay4 = 142,
    MaxCoherentNokReached = 143,
    BatchDone = 144,
    StartTriggerActive = 145,

    CompletedBatchBit0 = 251,
    CompletedBatchBit1 = 252,
    CompletedBatchBit2 = 253,
    CompletedBatchBit3 = 254,
    CompletedBatchBit4 = 255,
    CompletedBatchBit5 = 256,
    CompletedBatchBit6 = 257,

    RemainingBatchBit0 = 259,
    RemainingBatchBit1 = 260,
    RemainingBatchBit2 = 261,
    RemainingBatchBit3 = 262,
    RemainingBatchBit4 = 263,
    RemainingBatchBit5 = 264,
    RemainingBatchBit6 = 265,

    OpenProtocolCommandsDisabled = 275,
    CycleAbort = 276,
    EffectiveLoosening = 277,
    LogicRelay5 = 278,
    LogicRelay6 = 279,
    LogicRelay7 = 280,
    LogicRelay8 = 281,
    LogicRelay9 = 282,
    LogicRelay10 = 283,
    LockAtBatchDone = 284,

    BatteryLow = 287,
    BatteryEmpty = 288,
    ToolConnected = 289,
    NoToolConnected = 290,

    FunctionButton = 293,
    Rehit = 294,
    TighteningDisabled = 295,
    LooseningDisabled = 296,
    PositioningDisabled = 297,
    MotorTuningDisabled = 298,
    OpenEndTuningDisabled = 299,
    TrackingDisabled = 300,

    AutomaticMode = 302,
    PlusEmergencyMode = 303,
    WearIndicator = 304,
    DirectionAlert = 305,
    PlusBoltReworked = 306,
    LineStop = 307,
    RunningPSetBit9 = 308,
    ActiveXmlResultAck = 309,
    ToolInWorkSpace = 310,
    ToolInProductSpace = 311,
    XmlProtocolActive = 312,
    ToolEnabledByXml = 313,
    NeckingFailure = 314,
    PlusProtocolNotActive = 315,
    PlusNoTightening = 316,
    TagIdError = 317,
    JobAbortionInProgress = 318,
    StopTightening = 319,
    SlowDownTightening = 320,

    MiddleCourseTriggerActive = 351,
    FrontTriggerActive = 352,
    ReverseTriggerActive = 353,
    RunningJobBit9 = 354,
    ToolUnlocked = 355,
    /// <summary>
    /// Indicates that the connection to the Atlas Copco license server has been lost or the synchronization has failed. 
    /// The signal is cleared when the License manager synchronization has been done successfully
    /// </summary>
    LicenseServerConnectionLost = 356,
    /// <summary>
    /// Tightening not disabled by external source
    /// </summary>
    TighteningExternallyEnabled = 357,
    /// <summary>
    /// Tightening disabled by external source
    /// </summary>
    TighteningExternallyDisabled = 358,
    /// <summary>
    /// Loosening not disabled by external source
    /// </summary>
    LooseningExternallyEnabled = 359,
    /// <summary>
    /// Loosening disabled by external source
    /// </summary>
    LooseningExternallyDisabled = 360,
    /// <summary>
    /// Multistep tightening program has ended, torque has fallen below Program end torque configured.
    /// </summary>
    ProgramEnd = 361,
    /// <summary>
    /// Oil level supervision configured in the tool maintenance to remind the users when it is time to fill up oil in a pulse tool.
    /// </summary>
    PulseToolAlarmOilLevelEmpty = 362,
    /// <summary>
    /// Indicates high tightening time resulting in NOK tightening
    /// </summary>
    TighteningTimeHigh = 363,
    /// <summary>
    /// Indicates low tightening time resulting in NOK tightening
    /// </summary>
    TighteningTimeLow = 364,
    /// <summary>
    /// Output signal tracking the function button state. The signal is set when the function button is pressed and is cleared when the function button is released.
    /// </summary>
    ToolFunctionButtonPressed = 365
}

/// <summary>
/// Relay status. Used in <see cref="IOInterface.Mid0200"/>.
/// </summary>
pub enum RelayStatus
{
    Off = 0,
    On = 1,
    Flashing = 2,
    KeepCurrentStatus = 3
}

/// <summary>
/// Removal conditions. <see cref="UserInterface.Mid0111"/>.
/// </summary>
pub enum RemovalCondition
{
    AcknowledgeOrWaitExpirationTime = 0,
    Acknowledge = 1
}

/// <summary>
/// Reserved. Used in <see cref="Job.Mid0033"/> and <see cref="Job.Advanced.Mid0140"/>.
/// </summary>
pub enum Reserved
{
    E = 0,
    G = 1
}

/// <summary>
/// Result types. Used in <see cref="Tightening.Mid0061"/> and <see cref="Tightening.Mid0065"/>.
/// </summary>
pub enum ResultType
{
    Tightening = 1,
    Loosening = 2,
    BatchIncrement = 3,
    BatchDecrement = 4,
    BypassParameterSetResult = 5,
    AbortJobResult = 6,
    SyncTightening = 7,
    ReferenceSetup = 8,
    BatchReset = 9,
    JobRestart = 10
}

/// <summary>
/// Rotation directions. Used in <see cref="ParameterSet.Mid0013"/> and <see cref="ParameterSet.Mid0015"/>.
/// </summary>
pub enum RotationDirection
{
    Clockwise = 1,
    Counterclockwise = 2
}

/// <summary>
/// Status in work order. Used in <see cref="MultipleIdentifiers.IdentifierStatus"/>.
/// </summary>
pub enum StatusInWorkOrder
{
    NotAccepted = 0,
    Accepted = 1,
    Bypassed = 2,
    Reset = 3,
    Next = 4,
    Initial = 5
}

/// <summary>
/// Strategies. Used in <see cref="Tightening.Mid0061"/> and <see cref="Tightening.Mid0065"/>.
/// </summary>
pub enum Strategy
{
    TorqueControl = 1,
    TorqueControlAndAngleMonitoring = 2,
    TorqueControlAndAngleControl = 3,
    AngleControlAndTorqueMonitoring = 4,
    DSControl = 5,
    DSControlTorqueMonitoring = 6,
    ReverseAngle = 7,
    ReverseTorque = 8,
    ClickWrench = 9,
    RotateSpindleForward = 10,
    TorqueControlOrAngleControle = 11,
    RotateSpindleReverse = 12,
    HomePositionForward = 13,
    EPMonitoring = 14,
    Yield = 15,
    EPFixed = 16,
    EPControl = 17,
    EPAngleShutoff = 18,
    YieldOrTorqueControl = 19,
    SnugGradient = 20,
    ResidualTorqueAndTime = 21,
    ResidualAngleAndTime = 22,
    BreakawayPeak = 23,
    LoosAndTightening= 24,
    HomePositionReverse = 25,
    PVTCompWithSnug = 26,
    NoStrategy = 99
}

/// <summary>
/// If no subtype exists it will be set to 000
/// <para>For a Power Focus 4000 and PF 6000 system the valid subtypes are: </para>
/// <para>001 = a normal tightening system</para>
/// For a Power MACS 4000 system the valid subtypes are: 
/// <list type="bullet">
///     <item>001 = a normal tightening system</item>
///     <item>002 = a system running presses instead of spindles.</item>
/// </list>
/// </summary>
pub enum SystemSubType
{
    NoSubtypeExists = 0,
    NormalTighteningSystem = 1,
    SystemRunningPresses = 2
}

/// <summary>
/// The system type of the controller.
/// <para>Possible values are:</para>
/// <list type="bullet">
///     <item>000 = System type not set </item>
///     <item>001 = Power Focus 4000 </item>
///     <item>002 = Power MACS 4000 </item>
///     <item>003 = Power Focus 6000</item>
///     <item>004 = Micro Torque Focus 6000</item>
/// </list>
/// </summary>
pub enum SystemType
{
    SystemTypeNotSet = 0,
    PowerFocus4000 = 1,
    PowerMacs4000 = 2,
    PowerFocus6000 = 3,
    MicroTorqueFocus6000 = 4
}

/// <summary>
/// Tightening Directions. Used in <see cref="Tool.Mid0041"/>.
/// </summary>
pub enum TighteningDirection
{
    Clockwise = 0,
    Counterclockwise = 1
}

/// <summary>
/// Tightening value status. Used in <see cref="Tightening.Mid0061"/> and <see cref="Tightening.Mid0065"/>.
/// </summary>
pub enum TighteningValueStatus
{
    Low = 0,
    Ok = 1,
    High = 2
}

pub enum ToolHealth
{
    NotApplicable = 0,
    Ok = 1,
    Nok = 2
}

/// <summary>
/// Tool loosenings. Used in <see cref="Job.Mid0033"/> and <see cref="Job.Advanced.Mid0140"/>.
/// </summary>
pub enum ToolLoosening
{
    Enabled = 0,
    Disabled = 1,
    EnableOnlyOnNokTightenings = 2
}

/// <summary>
/// Tool status. Used in <see cref="ApplicationToolLocationSystem.Mid0265"/>.
/// </summary>
pub enum ToolStatus
{
    Operable = 1,
    Inoperable = 2
}

/// <summary>
/// Tool types. Used in <see cref="Tool.Mid0041"/>.
/// </summary>
pub enum ToolType
{
    NoTool = 0,
    S_Tool = 1,
    DS_Tool = 2,
    RefTransducer = 3,
    ST_Tool = 4,
    EP_Tool = 5,
    ETX_Tool = 6,
    SL_Tool = 7,
    DL_Tool = 8,
    IRC_Offline = 9,
    STB_Tool = 10,
    QST_Tool = 11,
    STT_Tool = 12,
    STWrench = 13,
    ES_Tool = 14,
    ESB = 15,
    SB = 16,
    SBPlus = 17,
    PST = 18,
    STR = 19,
    ETD_M = 20,
    ETD_MC = 21,
    ETD_MT = 22,
    QMC = 23,
    QMT = 24,
    BCV_RE = 25,
    BCP_REW = 26,
    E_LIT = 27,
    ISB = 28,
    ITB = 29,
    ITP = 30,
    QSHIELD_C = 31,
    DeltaWrench = 32,
    STRWrench = 33
}

/// <summary>
/// Torque status. Used in <see cref="PowerMACS.BoltData"/>.
/// </summary>
pub enum TorqueStatus
{
    Undefined = -1,
    BoltTorqueLow = 0,
    BoltTorqueOk = 1,
    BoltTorqueHigh = 2
}

/// <summary>
/// Torque values unit. Used in <see cref="Tightening.Mid0061"/> and <see cref="Tightening.Mid0065"/>.
/// </summary>
pub enum TorqueValuesUnit
{
    Nm = 1,
    LbfFt = 2,
    LbfIn = 3,
    Kpm = 4,
    KgfCm = 5,
    OzfIn = 6,
    
    #[deprecated(note="Percentage of torque was marked as obsolete since Specification 2.10.0")]
    Percentage = 7,
    Ncm = 8
}