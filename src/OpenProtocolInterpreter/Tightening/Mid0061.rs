/// <summary>
/// Last tightening result data
/// <para>Upload the last tightening result.</para>
/// <para>Message sent by: Controller</para>
/// <para>Answer: <see cref="Mid0062"/> Last tightening result data acknowledge</para>
/// </summary>

use crate::OpenProtocolInterpreter::MID::MidT;
use crate::OpenProtocolInterpreter::Header::{self, HeaderT};

pub(crate)  enum DataFields
{
    CellId,
    ChannelId,
    TorqueControllerName,
    VinNumber,
    JobId,
    ParameterSetId,
    BatchSize,
    BatchCounter,
    TighteningStatus,
    TorqueStatus,
    AngleStatus,
    TorqueMinLimit,
    TorqueMaxLimit,
    TorqueFinalTarget,
    Torque,
    AngleMinLimit,
    AngleMaxLimit,
    AngleFinalTarget,
    Angle,
    Timestamp,
    LastChangeInParameterSet,
    BatchStatus,
    TighteningId,
    //Rev 2
    Strategy,
    StrategyOptions,
    RundownAngleStatus,
    CurrentMonitoringStatus,
    SelftapStatus,
    PrevailTorqueMonitoringStatus,
    PrevailTorqueCompensateStatus,
    TighteningErrorStatus,
    RundownAngleMin,
    RundownAngleMax,
    RundownAngle,
    CurrentMonitoringMin,
    CurrentMonitoringMax,
    CurrentMonitoringValue,
    SelftapMin,
    SelftapMax,
    SelftapTorque,
    PrevailTorqueMonitoringMin,
    PrevailTorqueMonitoringMax,
    PrevailTorque,
    JobSequenceNumber,
    SyncTighteningId,
    ToolSerialNumber,
    //Rev 3
    ParameterSetName,
    TorqueValuesUnit,
    ResultType,
    //Rev 4
    IdentifierResultPart2,
    IdentifierResultPart3,
    IdentifierResultPart4,
    //Rev 5
    CustomerTighteningErrorCode,
    //Rev 6
    PrevailTorqueCompensateValue,
    TighteningErrorStatus2,
    //Rev 7
    CompensatedAngle,
    FinalAngleDecimal,
    //Rev 8
    StartFinalAngle,
    PostViewTorqueActivated,
    PostViewTorqueHigh,
    PostViewTorqueLow,
    //Rev 9
    CurrentMonitoringAmp,
    CurrentMonitoringAmpMin,
    CurrentMonitoringAmpMax,
    //Rev 10
    AngleNumeratorScaleFactor,
    AngleDenominatorScaleFactor,
    OverallAngleStatus,
    OverallAngleMin,
    OverallAngleMax,
    OverallAngle,
    PeakTorque,
    ResidualBreakawayTorque,
    StartRundownAngle,
    RundownAngleComplete,
    //Rev 11
    ClickTorque,
    ClickAngle,
    //Rev 998 (Go over rev 7)
    NumberOfStagesInMultistage,
    NumberOfStageResults,
    StageResult
    //Rev 999 => all registered
}

#[derive(Clone)]
pub struct Mid0061T { //:Mid, ITightening, IController, IAcknowledgeable<Mid0062>
    pub mid:MidT,
}

impl Mid0061T {
    pub const MID:i32 = 61;

    pub fn new()->Self
    {
        Self::new_rev(Header::DEFAULT_REVISION)
    }

    pub fn new_header(header:HeaderT)->Self
    {
        Mid0061T { mid: MidT::new(header) }
    }

    pub fn new_rev(revision:i32) ->Self {
        let mut h:HeaderT = HeaderT::default();
        h.mid = Self::MID;
        h.revision = revision;
        Self::new_header(h)
    }
}