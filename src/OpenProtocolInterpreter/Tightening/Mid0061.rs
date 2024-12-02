use chrono::{DateTime, Local};

use crate::OpenProtocolInterpreter::Enums;
use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;
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
    //pub StrategyOptions:Enums::StrategyOptions;
}

impl Mid0061T {
    pub const MID:i32 = 61;

    pub fn cell_id(&mut self) ->i32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::CellId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_cell_id(&mut self, value:i32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::CellId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn channel_id(&mut self) ->i32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::ChannelId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_channel_id(&mut self, value:i32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::ChannelId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn torque_controller_name(&mut self) ->String {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TorqueControllerName as i32).value
    }

    pub fn set_torque_controller_name(&mut self, value:String) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TorqueControllerName as i32).set_value(value);
    }

    pub fn vin_number(&mut self) ->String {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::VinNumber as i32).value
    }

    pub fn set_vin_number(&mut self, value:String) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::VinNumber as i32).set_value(value);
    }

    pub fn job_id(&mut self) ->i32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::JobId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_job_id(&mut self, value:i32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::JobId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn parameter_set_id(&mut self) ->i32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::ParameterSetId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_parameter_set_id(&mut self, value:i32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::ParameterSetId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn batch_size(&mut self) ->i32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::BatchSize as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_batch_size(&mut self, value:i32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::BatchSize as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn batch_counter(&mut self) ->i32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::BatchCounter as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_batch_counter(&mut self, value:i32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::BatchCounter as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn tightening_status(&mut self) ->bool {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TighteningStatus as i32).get_value(OpenProtocolConvertT::string_to_bool)
    }

    pub fn set_tightening_status(&mut self, value:bool) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TighteningStatus as i32).set_value2::<bool>(OpenProtocolConvertT::tp_bool_to_string, value);
    }

    pub fn torque_status(&mut self) ->Enums::TighteningValueStatus {
        let val = self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TorqueStatus as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let tvs:Enums::TighteningValueStatus = unsafe { ::std::mem::transmute(val) };

        tvs
    }

    pub fn set_torque_status(&mut self, value:Enums::TighteningValueStatus) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TorqueStatus as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    pub fn angle_status(&mut self) ->Enums::TighteningValueStatus {
        let val = self.mid.get_field(self.clone().get_current_revision_index(), DataFields::AngleStatus as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let tvs:Enums::TighteningValueStatus = unsafe { ::std::mem::transmute(val) };

        tvs
    }

    pub fn set_angle_status(&mut self, value:Enums::TighteningValueStatus) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::AngleStatus as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    /*
            public decimal TorqueMinLimit
        {
            get => GetField(GetCurrentRevisionIndex(), DataFields.TorqueMinLimit).GetValue(OpenProtocolConvert.ToTruncatedDecimal);
            set => GetField(GetCurrentRevisionIndex(), DataFields.TorqueMinLimit).SetValue(OpenProtocolConvert.TruncatedDecimalToString, value);
        }
     */
    pub fn torque_min_limit(&mut self) ->f32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TorqueMinLimit as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_torque_min_limit(&mut self, value:f32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TorqueMinLimit as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn torque_max_limit(&mut self) ->f32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TorqueMaxLimit as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_torque_max_limit(&mut self, value:f32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TorqueMaxLimit as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn torque_final_target(&mut self) ->f32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TorqueFinalTarget as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_torque_final_target(&mut self, value:f32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TorqueFinalTarget as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn torque(&mut self) ->f32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::Torque as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_torque(&mut self, value:f32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::Torque as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn angle_min_limit(&mut self) ->i32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::AngleMinLimit as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_angle_min_limit(&mut self, value:i32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::AngleMinLimit as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn angle_max_limit(&mut self) ->i32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::AngleMaxLimit as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_angle_max_limit(&mut self, value:i32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::AngleMaxLimit as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn angle_final_target(&mut self) ->i32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::AngleFinalTarget as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_angle_final_target(&mut self, value:i32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::AngleFinalTarget as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn angle(&mut self) ->i32 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::Angle as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_angle(&mut self, value:i32) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::Angle as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn timestamp(&mut self) ->DateTime<Local>
    {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::Timestamp as i32).get_value(OpenProtocolConvertT::string_to_date_time)
    }

    pub fn set_timestamp(&mut self, value:DateTime<Local>) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::Timestamp as i32).set_value2::<DateTime<Local>>(OpenProtocolConvertT::tp_date_time_to_string, value);
    }

    pub fn last_change_in_parameter_set(&mut self) ->DateTime<Local>
    {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::LastChangeInParameterSet as i32).get_value(OpenProtocolConvertT::string_to_date_time)
    }

    pub fn set_last_change_in_parameter_set(&mut self, value:DateTime<Local>) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::LastChangeInParameterSet as i32).set_value2::<DateTime<Local>>(OpenProtocolConvertT::tp_date_time_to_string, value);
    }

    pub fn batch_status(&mut self) ->Enums::BatchStatus {
        let val = self.mid.get_field(self.clone().get_current_revision_index(), DataFields::BatchStatus as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let bs:Enums::BatchStatus = unsafe { ::std::mem::transmute(val) };

        bs
    }

    pub fn set_batch_status(&mut self, value:Enums::BatchStatus) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::BatchStatus as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    pub fn tightening_id(&mut self) ->i64 {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TighteningId as i32).get_value(OpenProtocolConvertT::string_to_int64)
    }

    pub fn set_tightening_id(&mut self, value:i64) {
        self.mid.get_field(self.clone().get_current_revision_index(), DataFields::TighteningId as i32).set_value2::<i64>(OpenProtocolConvertT::tp_i64_to_string, value);
    }

    //Rev 2 Addition
    pub fn strategy(&mut self) ->Enums::Strategy {
        let val = self.mid.get_field(2, DataFields::Strategy as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let strg:Enums::Strategy = unsafe { ::std::mem::transmute(val) };

        strg
    }

    pub fn set_strategy(&mut self, value:Enums::Strategy) {
        self.mid.get_field(2, DataFields::Strategy as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    //public StrategyOptions StrategyOptions { get; set; }

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

    /// <summary>
    /// Obtain which revision we will work with for shared properties
    /// (since rev 1, 2 and 999 are way too different, they are processed in different datafields)
    /// </summary>
    /// <returns>Datafield Revision Index</returns>
    fn get_current_revision_index(&mut self)->i32 {
        if self.mid.header.revision == 999 {
            return 999;
        }

        if self.mid.header.revision > 1 {
            return 2;
        }
        return 1;
    }
}