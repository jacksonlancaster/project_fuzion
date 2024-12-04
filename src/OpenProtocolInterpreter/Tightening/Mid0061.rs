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
use super::StageResult::StageResultT;
use super::StrategyOptions::StrategyOptionsT;
use super::TighteningErrorStatus::{TighteningErrorStatus2T, TighteningErrorStatusT};

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
    pub strategy_options:StrategyOptionsT,
    pub tightening_error_status:TighteningErrorStatusT,
    pub tightening_error_status2:TighteningErrorStatus2T,
    pub stage_results:Vec<StageResultT>,
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

    /*TBD */
    //public StrategyOptions StrategyOptions { get; set; }

    pub fn rundown_angle_status(&mut self) ->Enums::TighteningValueStatus {
        let val = self.mid.get_field(2, DataFields::RundownAngleStatus as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let tvs:Enums::TighteningValueStatus = unsafe { ::std::mem::transmute(val) };

        tvs
    }

    pub fn set_rundown_angle_status(&mut self, value:Enums::TighteningValueStatus) {
        self.mid.get_field(2, DataFields::RundownAngleStatus as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    pub fn current_monitoring_status(&mut self) ->Enums::TighteningValueStatus {
        let val = self.mid.get_field(2, DataFields::CurrentMonitoringStatus as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let tvs:Enums::TighteningValueStatus = unsafe { ::std::mem::transmute(val) };

        tvs
    }

    pub fn set_current_monitoring_status(&mut self, value:Enums::TighteningValueStatus) {
        self.mid.get_field(2, DataFields::CurrentMonitoringStatus as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    pub fn selftap_status(&mut self) ->Enums::TighteningValueStatus {
        let val = self.mid.get_field(2, DataFields::SelftapStatus as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let tvs:Enums::TighteningValueStatus = unsafe { ::std::mem::transmute(val) };

        tvs
    }

    pub fn set_selftap_status(&mut self, value:Enums::TighteningValueStatus) {
        self.mid.get_field(2, DataFields::SelftapStatus as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

     pub fn prevail_torque_monitoring_status(&mut self) ->Enums::TighteningValueStatus {
        let val = self.mid.get_field(2, DataFields::PrevailTorqueMonitoringStatus as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let tvs:Enums::TighteningValueStatus = unsafe { ::std::mem::transmute(val) };

        tvs
    }

    pub fn set_prevail_torque_monitoring_status(&mut self, value:Enums::TighteningValueStatus) {
        self.mid.get_field(2, DataFields::PrevailTorqueMonitoringStatus as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    pub fn prevail_torque_compensate_status(&mut self) ->Enums::TighteningValueStatus {
        let val = self.mid.get_field(2, DataFields::PrevailTorqueCompensateStatus as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let tvs:Enums::TighteningValueStatus = unsafe { ::std::mem::transmute(val) };

        tvs
    }

    pub fn set_prevail_torque_compensate_status(&mut self, value:Enums::TighteningValueStatus) {
        self.mid.get_field(2, DataFields::PrevailTorqueCompensateStatus as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    /*TBD */
    //public TighteningErrorStatus TighteningErrorStatus { get; set; }

    pub fn rundown_angle_min(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::RundownAngleMin as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_rundown_angle_min(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::RundownAngleMin as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn rundown_angle_max(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::RundownAngleMax as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_rundown_angle_max(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::RundownAngleMax as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn rundown_angle(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::RundownAngle as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_rundown_angle(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::RundownAngle as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn current_monitoring_min(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::CurrentMonitoringMin as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_current_monitoring_min(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::CurrentMonitoringMin as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn current_monitoring_max(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::CurrentMonitoringMax as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_current_monitoring_max(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::CurrentMonitoringMax as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn current_monitoring_value(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::CurrentMonitoringValue as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_current_monitoring_value(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::CurrentMonitoringValue as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn selftap_min(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::SelftapMin as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_selftap_min(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::SelftapMin as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn selftap_max(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::SelftapMax as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_selftap_max(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::SelftapMax as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn selftap_torque(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::SelftapTorque as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_selftap_torque(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::SelftapTorque as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn prevail_torque_monitoring_min(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::PrevailTorqueMonitoringMin as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_prevail_torque_monitoring_min(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::PrevailTorqueMonitoringMin as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn prevail_torque_monitoring_max(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::PrevailTorqueMonitoringMax as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_prevail_torque_monitoring_max(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::PrevailTorqueMonitoringMax as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn prevail_torque(&mut self) ->f32 {
        self.mid.get_field(2, DataFields::PrevailTorque as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_prevail_torque(&mut self, value:f32) {
        self.mid.get_field(2, DataFields::PrevailTorque as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn job_sequence_number(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::JobSequenceNumber as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_job_sequence_number(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::JobSequenceNumber as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn sync_tightening_id(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::SyncTighteningId as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_sync_tightening_id(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::SyncTighteningId as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn tool_serial_number(&mut self) ->i32 {
        self.mid.get_field(2, DataFields::ToolSerialNumber as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_tool_serial_number(&mut self, value:i32) {
        self.mid.get_field(2, DataFields::ToolSerialNumber as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    //Rev 3 Addition
    pub fn parameter_set_name(&mut self) ->String {
        self.mid.get_field(3, DataFields::ParameterSetName as i32).value
    }

    pub fn set_parameter_set_name(&mut self, value:String) {
        self.mid.get_field(3, DataFields::ParameterSetName as i32).set_value(value);
    }

    pub fn torque_values_unit(&mut self) ->Enums::TorqueValuesUnit {
        let val = self.mid.get_field(3, DataFields::TorqueValuesUnit as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let tvu:Enums::TorqueValuesUnit = unsafe { ::std::mem::transmute(val) };

        tvu
    }

    pub fn set_torque_values_unit(&mut self, value:Enums::TorqueValuesUnit) {
        self.mid.get_field(3, DataFields::TorqueValuesUnit as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    pub fn result_type(&mut self) ->Enums::ResultType {
        let val = self.mid.get_field(3, DataFields::ResultType as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let tvu:Enums::ResultType = unsafe { ::std::mem::transmute(val) };

        tvu
    }

    pub fn set_result_type(&mut self, value:Enums::ResultType) {
        self.mid.get_field(3, DataFields::ResultType as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    //Rev 4 addition
    pub fn identifier_result_part2(&mut self) ->String {
        self.mid.get_field(4, DataFields::IdentifierResultPart2 as i32).value
    }

    pub fn set_identifier_result_part2(&mut self, value:String) {
        self.mid.get_field(4, DataFields::IdentifierResultPart2 as i32).set_value(value);
    }

    pub fn identifier_result_part3(&mut self) ->String {
        self.mid.get_field(4, DataFields::IdentifierResultPart3 as i32).value
    }

    pub fn set_identifier_result_part3(&mut self, value:String) {
        self.mid.get_field(4, DataFields::IdentifierResultPart3 as i32).set_value(value);
    }

    pub fn IdentifierResultPart4(&mut self) ->String {
        self.mid.get_field(4, DataFields::IdentifierResultPart4 as i32).value
    }

    pub fn set_IdentifierResultPart4(&mut self, value:String) {
        self.mid.get_field(4, DataFields::IdentifierResultPart4 as i32).set_value(value);
    }

    //Rev 5 addition
    pub fn customer_tightening_error_code(&mut self) ->String {
        self.mid.get_field(5, DataFields::CustomerTighteningErrorCode as i32).value
    }

    pub fn set_customer_tightening_error_code(&mut self, value:String) {
        self.mid.get_field(5, DataFields::CustomerTighteningErrorCode as i32).set_value(value);
    }

    //Rev 6 Addition
    pub fn prevail_torque_compensate_value(&mut self) ->f32 {
        self.mid.get_field(6, DataFields::PrevailTorqueCompensateValue as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_prevail_torque_compensate_value(&mut self, value:f32) {
        self.mid.get_field(6, DataFields::PrevailTorqueCompensateValue as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    /* TBD */
    //public TighteningErrorStatus2 TighteningErrorStatus2 { get; set; }
  
    //Rev 7 addition
    pub fn compensated_angle(&mut self) ->f32 {
        self.mid.get_field(7, DataFields::CompensatedAngle as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_compensated_angle(&mut self, value:f32) {
        self.mid.get_field(7, DataFields::CompensatedAngle as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn final_angle_decimal(&mut self) ->f32 {
        self.mid.get_field(7, DataFields::FinalAngleDecimal as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_final_angle_decimal(&mut self, value:f32) {
        self.mid.get_field(7, DataFields::FinalAngleDecimal as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }
    //Rev 8 addition
    pub fn start_final_angle(&mut self) ->f32 {
        self.mid.get_field(8, DataFields::StartFinalAngle as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_start_final_angle(&mut self, value:f32) {
        self.mid.get_field(8, DataFields::StartFinalAngle as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn post_view_torque_activated(&mut self) ->Enums::PostViewTorque {
        let val = self.mid.get_field(8, DataFields::PostViewTorqueActivated as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let tvu:Enums::PostViewTorque = unsafe { ::std::mem::transmute(val) };

        tvu
    }

    pub fn set_post_view_torque_activated(&mut self, value:Enums::PostViewTorque) {
        self.mid.get_field(8, DataFields::PostViewTorqueActivated as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    pub fn post_view_torque_high(&mut self) ->f32 {
        self.mid.get_field(8, DataFields::PostViewTorqueHigh as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_post_view_torque_high(&mut self, value:f32) {
        self.mid.get_field(8, DataFields::PostViewTorqueHigh as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn post_view_torque_low(&mut self) ->f32 {
        self.mid.get_field(8, DataFields::PostViewTorqueLow as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_post_view_torque_low(&mut self, value:f32) {
        self.mid.get_field(8, DataFields::PostViewTorqueLow as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }
    //Rev 9 addition
    pub fn current_monitoring_ampere(&mut self) ->f32 {
        self.mid.get_field(9, DataFields::CurrentMonitoringAmp as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_current_monitoring_ampere(&mut self, value:f32) {
        self.mid.get_field(9, DataFields::CurrentMonitoringAmp as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn current_monitoring_ampere_min(&mut self) ->f32 {
        self.mid.get_field(9, DataFields::CurrentMonitoringAmpMin as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_current_monitoring_ampere_min(&mut self, value:f32) {
        self.mid.get_field(9, DataFields::CurrentMonitoringAmpMin as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn current_monitoring_ampere_max(&mut self) ->f32 {
        self.mid.get_field(9, DataFields::CurrentMonitoringAmpMax as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_current_monitoring_ampere_max(&mut self, value:f32) {
        self.mid.get_field(9, DataFields::CurrentMonitoringAmpMax as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }
    
    //Rev 10 addition
    pub fn angle_numerator_scale_factor(&mut self) ->i32 {
        self.mid.get_field(10, DataFields::AngleNumeratorScaleFactor as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_angle_numerator_scale_factor(&mut self, value:i32) {
        self.mid.get_field(10, DataFields::AngleNumeratorScaleFactor as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn angle_denominator_scale_factor(&mut self) ->i32 {
        self.mid.get_field(10, DataFields::AngleDenominatorScaleFactor as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_angle_denominator_scale_factor(&mut self, value:i32) {
        self.mid.get_field(10, DataFields::AngleDenominatorScaleFactor as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn overall_angle_status(&mut self) ->Enums::TighteningValueStatus {
        let val = self.mid.get_field(10, DataFields::OverallAngleStatus as i32).get_value(OpenProtocolConvertT::string_to_int32);
        let tvs:Enums::TighteningValueStatus = unsafe { ::std::mem::transmute(val) };

        tvs
    }

    pub fn set_overall_angle_status(&mut self, value:Enums::TighteningValueStatus) {
        self.mid.get_field(10, DataFields::OverallAngleStatus as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value as i32);
    }

    pub fn overall_angle_min(&mut self) ->i32 {
        self.mid.get_field(10, DataFields::OverallAngleMin as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_overall_angle_min(&mut self, value:i32) {
        self.mid.get_field(10, DataFields::OverallAngleMin as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn overall_angle_max(&mut self) ->i32 {
        self.mid.get_field(10, DataFields::OverallAngleMax as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_overall_angle_max(&mut self, value:i32) {
        self.mid.get_field(10, DataFields::OverallAngleMax as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn overall_angle(&mut self) ->i32 {
        self.mid.get_field(10, DataFields::OverallAngle as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_overall_angle(&mut self, value:i32) {
        self.mid.get_field(10, DataFields::OverallAngle as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn peak_torque(&mut self) ->f32 {
        self.mid.get_field(10, DataFields::PeakTorque as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_peak_torque(&mut self, value:f32) {
        self.mid.get_field(10, DataFields::PeakTorque as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn residual_breakaway_torque(&mut self) ->f32 {
        self.mid.get_field(10, DataFields::ResidualBreakawayTorque as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_residual_breakaway_torque(&mut self, value:f32) {
        self.mid.get_field(10, DataFields::ResidualBreakawayTorque as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn start_rundown_angle(&mut self) ->f32 {
        self.mid.get_field(10, DataFields::StartRundownAngle as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_start_rundown_angle(&mut self, value:f32) {
        self.mid.get_field(10, DataFields::StartRundownAngle as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }
    
    pub fn rundown_angle_complete(&mut self) ->f32 {
        self.mid.get_field(10, DataFields::RundownAngleComplete as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_rundown_angle_complete(&mut self, value:f32) {
        self.mid.get_field(10, DataFields::RundownAngleComplete as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    //Rev 11
    pub fn click_torque(&mut self) ->f32 {
        self.mid.get_field(11, DataFields::ClickTorque as i32).get_value(OpenProtocolConvertT::string_to_truncated_decimal)
    }

    pub fn set_click_torque(&mut self, value:f32) {
        self.mid.get_field(11, DataFields::ClickTorque as i32).set_value2::<f32>(OpenProtocolConvertT::tp_truncated_decimal_to_string, value);
    }

    pub fn click_angle(&mut self) ->i32 {
        self.mid.get_field(11, DataFields::ClickAngle as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_click_angle(&mut self, value:i32) {
        self.mid.get_field(11, DataFields::ClickAngle as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    //Rev 998 addition 
    pub fn number_of_stages_in_multistage(&mut self) ->i32 {
        self.mid.get_field(998, DataFields::NumberOfStagesInMultistage as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_number_of_stages_in_multistage(&mut self, value:i32) {
        self.mid.get_field(998, DataFields::NumberOfStagesInMultistage as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn number_of_stage_results(&mut self) ->i32 {
        self.mid.get_field(998, DataFields::NumberOfStageResults as i32).get_value(OpenProtocolConvertT::string_to_int32)
    }

    pub fn set_number_of_stage_results(&mut self, value:i32) {
        self.mid.get_field(998, DataFields::NumberOfStageResults as i32).set_value2::<i32>(OpenProtocolConvertT::tp_i32_to_string, value);
    }

    pub fn new()->Self
    {
        Self::new_rev(Header::DEFAULT_REVISION)
    }

    pub fn new_header(header:HeaderT)->Self
    {
        Mid0061T { mid: MidT::new(header), stage_results:Vec::new() }
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