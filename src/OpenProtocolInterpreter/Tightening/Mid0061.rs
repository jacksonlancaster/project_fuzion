use std::collections::HashMap;

use chrono::{DateTime, Local};

use crate::OpenProtocolInterpreter::DataField::DataFieldT;
use crate::OpenProtocolInterpreter::Enums::{self, PaddingOrientation};
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

#[derive(Clone, Default)]
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
        Mid0061T { mid: MidT::new(header), stage_results:Vec::new(), ..Default::default()}
    }

    pub fn new_rev(revision:i32) ->Self {
        let mut h:HeaderT = HeaderT::default();
        h.mid = Self::MID;
        h.revision = revision;
        Self::new_header(h)
    }

    pub(crate) fn build_header(&mut self)->String
        {
            if !self.mid.revisions_by_fields.is_empty() {
                self.mid.header.length = 20;
                self.mid.header.revision = if self.mid.header.revision > 0  {self.mid.header.revision} else {1};
                if self.mid.header.revision == 1 || self.mid.header.revision == 999 {
                    let data_fields = self.mid.revisions_by_fields.get(&self.mid.header.revision);
                    for data_field in data_fields.unwrap()  {
                        self.mid.header.length += (if data_field.has_prefix  {2} else {0}) + data_field.size;
                    }
                } else {
                    let process_until:i32 = if self.mid.header.revision != 998  {self.mid.header.revision} else {6};
                    let mut i = 2;
                    while i <= process_until {
                        let data_fields = self.mid.revisions_by_fields.get(&i);
                        for data_field in data_fields.unwrap() {
                            self.mid.header.length += (if data_field.has_prefix  {2} else {0}) + data_field.size;
                        }
                        i +=1;
                    }

                    if self.mid.header.revision == 998 {
                       self.mid.get_field(998, DataFields::StageResult as i32).size = self.stage_results.len() as i32 * 11;
                       let k = 998;
                       let data_fields = self.mid.revisions_by_fields.get(&k);
                        for data_field in data_fields.unwrap() {
                            self.mid.header.length += (if data_field.has_prefix  {2} else {0}) + data_field.size;
                        }
                    }
                }
            }
            self.mid.header.to_string()
        }

    pub fn set_header(&mut self, hdr:HeaderT) {
        self.mid.header = hdr
    }

    pub fn process_header(&mut self, package:String)->HeaderT {
        self.mid.process_header(package)
    }

    pub fn pack(&mut self)->String {
        let mut builder = String::new();
        let mut prefix_index = 1;
        if self.mid.header.revision > 1 && self.mid.header.revision != 999 {
            self.mid.get_field(2, DataFields::StrategyOptions as i32).set_value(self.strategy_options.pack());
            self.mid.get_field(2, DataFields::TighteningErrorStatus as i32).set_value(self.tightening_error_status.pack());

            if self.mid.header.revision > 5 {
                self.mid.get_field(6, DataFields::TighteningErrorStatus2 as i32).set_value(self.tightening_error_status2.pack());
            }

            if self.mid.header.revision == 998 {
                self.set_number_of_stage_results(self.stage_results.len() as i32);
                let mut stage_result_field = self.mid.get_field(998, DataFields::StageResult as i32);
                stage_result_field.size = self.stage_results.len() as i32 * 11;
                stage_result_field.set_value(self.pack_stage_results());
            }

            builder.push_str(self.build_header().as_str());
            let process_until_revision:i32 = if self.mid.header.revision != 998 {self.mid.header.revision} else {6};
            let mut revision = 2;
            while revision <= process_until_revision {
                builder.push_str(self.mid.pack2(revision, &mut prefix_index).as_str());
                revision +=1;
            }

            if self.mid.header.revision == 998 {
                builder.push_str(self.mid.pack2(998, &mut prefix_index).as_str());
            }
        }
        else
        {
            builder.push_str(self.build_header().as_str());
            builder.push_str(self.mid.pack2(self.mid.header.revision, &mut prefix_index).as_str());
        }

        builder
    }


    pub(crate) fn  process_data_fields(&mut self, package:String) {
        if self.mid.header.revision == 1 || self.mid.header.revision == 999 {
            self.mid.process_data_fields2(self.mid.header.revision, package.clone());
        }
        else
        {
            let mut process_until_revision = self.mid.header.revision;
            if self.mid.header.revision == 998 {
                process_until_revision = 6;
                let mut stage_result_field = self.mid.get_field(998, DataFields::StageResult as i32);
                stage_result_field.size = self.mid.header.length - stage_result_field.index - 2;
                self.mid.process_data_fields2(998, package.clone());
                self.stage_results = StageResultT::parse_all(stage_result_field.value).to_vec();
            }

            let mut revision = 2;
            while revision <= process_until_revision {
                self.mid.process_data_fields2(revision, package.clone());
                revision +=1;
            }

            let strategy_options_field = self.mid.get_field(2, DataFields::StrategyOptions as i32);
            self.strategy_options = StrategyOptionsT::parse_from_str(strategy_options_field.value);

            let tightening_error_status_field = self.mid.get_field(2, DataFields::TighteningErrorStatus as i32);
            self.tightening_error_status = TighteningErrorStatusT::parse_from_str(tightening_error_status_field.value);

            if self.mid.header.revision > 5 {
                let tightening_error_status2_field = self.mid.get_field(6, DataFields::TighteningErrorStatus2 as i32);
                self.tightening_error_status2 = TighteningErrorStatus2T::parse_from_str(tightening_error_status2_field.value);
            }
        }
    }

    pub(crate) fn pack_stage_results(&mut self)->String {
        let mut builder = String::new();
        for mut v in self.stage_results.clone() {
            builder.push_str(v.pack().as_str());
        }

        builder
    }
    
    pub(crate) fn register_datafields()->HashMap<i32, Vec<DataFieldT>> {
        let mut hm:HashMap<i32, Vec<DataFieldT>>  = HashMap::new();
        
        let mut v1:Vec<DataFieldT>= Vec::new();
        v1.push(DataFieldT::number(DataFields::CellId as i32, 20, 4, None));
        v1.push(DataFieldT::number(DataFields::ChannelId as i32, 26, 2, None));
        v1.push(DataFieldT::string2(DataFields::TorqueControllerName as i32, 30, 25, None));
        v1.push(DataFieldT::string2(DataFields::VinNumber as i32, 57, 25, None));
        v1.push(DataFieldT::number(DataFields::JobId as i32, 84, 2, None));
        v1.push(DataFieldT::number(DataFields::ParameterSetId as i32, 88, 3, None));
        v1.push(DataFieldT::number(DataFields::BatchSize as i32, 93, 4, None));
        v1.push(DataFieldT::number(DataFields::BatchCounter as i32, 99, 4, None));
        v1.push(DataFieldT::number(DataFields::TighteningStatus as i32, 105, 1, None));
        v1.push(DataFieldT::number(DataFields::TorqueStatus as i32, 108, 1, None));
        v1.push(DataFieldT::number(DataFields::AngleStatus as i32, 111, 1, None));
        v1.push(DataFieldT::number(DataFields::TorqueMinLimit as i32, 114, 6, None));
        v1.push(DataFieldT::number(DataFields::TorqueMaxLimit as i32, 122, 6, None));
        v1.push(DataFieldT::number(DataFields::TorqueFinalTarget as i32, 130, 6, None));
        v1.push(DataFieldT::number(DataFields::Torque as i32, 138, 6, None));
        v1.push(DataFieldT::number(DataFields::AngleMinLimit as i32, 146, 5, None));
        v1.push(DataFieldT::number(DataFields::AngleMaxLimit as i32, 153, 5, None));
        v1.push(DataFieldT::number(DataFields::AngleFinalTarget as i32, 160, 5, None));
        v1.push(DataFieldT::number(DataFields::Angle as i32, 167, 5, None));
        v1.push(DataFieldT::timestamp(DataFields::Timestamp as i32, 174, None));
        v1.push(DataFieldT::timestamp(DataFields::LastChangeInParameterSet as i32, 195, None));
        v1.push(DataFieldT::number(DataFields::BatchStatus as i32, 216, 1, None));
        v1.push(DataFieldT::number(DataFields::TighteningId as i32, 219, 10, None));
        hm.insert(1, v1);

        let mut v2:Vec<DataFieldT>= Vec::new();
        v2.push(DataFieldT::number(DataFields::CellId as i32, 20, 4, None));
        v2.push(DataFieldT::number(DataFields::ChannelId as i32, 26, 2, None));
        v2.push(DataFieldT::string2(DataFields::TorqueControllerName as i32, 30, 25, None));
        v2.push(DataFieldT::string2(DataFields::VinNumber as i32, 57, 25, None));
        v2.push(DataFieldT::number(DataFields::JobId as i32, 84, 4, None));
        v2.push(DataFieldT::number(DataFields::ParameterSetId as i32, 90, 3, None));
        v2.push(DataFieldT::number(DataFields::Strategy as i32, 95, 2, None));
        v2.push(DataFieldT::new3(DataFields::StrategyOptions as i32, 99, 5, '0', Some(PaddingOrientation::LeftPadded), None));
        v2.push(DataFieldT::number(DataFields::BatchSize as i32, 106, 4, None));
        v2.push(DataFieldT::number(DataFields::BatchCounter as i32, 112, 4, None));
        v2.push(DataFieldT::number(DataFields::TighteningStatus as i32, 118, 1, None));
        v2.push(DataFieldT::number(DataFields::BatchStatus as i32, 121, 1, None));
        v2.push(DataFieldT::number(DataFields::TorqueStatus as i32, 124, 1, None));
        v2.push(DataFieldT::number(DataFields::AngleStatus as i32, 127, 1, None));
        v2.push(DataFieldT::number(DataFields::RundownAngleStatus as i32, 130, 1, None));
        v2.push(DataFieldT::number(DataFields::CurrentMonitoringStatus as i32, 133, 1, None));
        v2.push(DataFieldT::number(DataFields::SelftapStatus as i32, 136, 1, None));
        v2.push(DataFieldT::number(DataFields::PrevailTorqueMonitoringStatus as i32, 139, 1, None));
        v2.push(DataFieldT::number(DataFields::PrevailTorqueCompensateStatus as i32, 142, 1, None));
        v2.push(DataFieldT::new3(DataFields::TighteningErrorStatus as i32, 145, 10, '0', Some(PaddingOrientation::LeftPadded), None));
        v2.push(DataFieldT::number(DataFields::TorqueMinLimit as i32, 157, 6, None));
        v2.push(DataFieldT::number(DataFields::TorqueMaxLimit as i32, 165, 6, None));
        v2.push(DataFieldT::number(DataFields::TorqueFinalTarget as i32, 173, 6, None));
        v2.push(DataFieldT::number(DataFields::Torque as i32, 181, 6, None));
        v2.push(DataFieldT::number(DataFields::AngleMinLimit as i32, 189, 5, None));
        v2.push(DataFieldT::number(DataFields::AngleMaxLimit as i32, 196, 5, None));
        v2.push(DataFieldT::number(DataFields::AngleFinalTarget as i32, 203, 5, None));
        v2.push(DataFieldT::number(DataFields::Angle as i32, 210, 5, None));
        v2.push(DataFieldT::number(DataFields::RundownAngleMin as i32, 217, 5, None));
        v2.push(DataFieldT::number(DataFields::RundownAngleMax as i32, 224, 5, None));
        v2.push(DataFieldT::number(DataFields::RundownAngle as i32, 231, 5, None));
        v2.push(DataFieldT::number(DataFields::CurrentMonitoringMin as i32, 238, 3, None));
        v2.push(DataFieldT::number(DataFields::CurrentMonitoringMax as i32, 243, 3, None));
        v2.push(DataFieldT::number(DataFields::CurrentMonitoringValue as i32, 248, 3, None));
        v2.push(DataFieldT::number(DataFields::SelftapMin as i32, 253, 6, None));
        v2.push(DataFieldT::number(DataFields::SelftapMax as i32, 261, 6  , None));
        v2.push(DataFieldT::number(DataFields::SelftapTorque as i32, 269, 6, None));
        v2.push(DataFieldT::number(DataFields::PrevailTorqueMonitoringMin as i32, 277, 6, None));
        v2.push(DataFieldT::number(DataFields::PrevailTorqueMonitoringMax as i32, 285, 6, None));
        v2.push(DataFieldT::number(DataFields::PrevailTorque as i32, 293, 6, None));
        v2.push(DataFieldT::number(DataFields::TighteningId as i32, 301, 10, None));
        v2.push(DataFieldT::number(DataFields::JobSequenceNumber as i32, 313, 5, None));
        v2.push(DataFieldT::number(DataFields::SyncTighteningId as i32, 320, 5, None));
        v2.push(DataFieldT::string2(DataFields::ToolSerialNumber as i32, 327, 14, None));
        v2.push(DataFieldT::timestamp(DataFields::Timestamp as i32, 343, None));
        v2.push(DataFieldT::timestamp(DataFields::LastChangeInParameterSet as i32, 364, None));
        hm.insert(2, v2);

        let mut v3:Vec<DataFieldT>= Vec::new();
        v3.push(DataFieldT::string2(DataFields::ParameterSetName as i32, 385, 25, None));
        v3.push(DataFieldT::number(DataFields::TorqueValuesUnit as i32, 412, 1, None));
        v3.push(DataFieldT::number(DataFields::ResultType as i32, 415, 2, None));
        hm.insert(3, v3);

        let mut v4:Vec<DataFieldT>= Vec::new();
        v4.push(DataFieldT::string2(DataFields::IdentifierResultPart2 as i32, 419, 25, None));
        v4.push(DataFieldT::string2(DataFields::IdentifierResultPart3 as i32, 446, 25, None));
        v4.push(DataFieldT::string2(DataFields::IdentifierResultPart4 as i32, 473, 25, None));
        hm.insert(4, v4);

        
        let mut v5:Vec<DataFieldT>= Vec::new();
        v5.push(DataFieldT::string2(DataFields::CustomerTighteningErrorCode as i32, 500, 4, None));
        hm.insert(5, v5);

        let mut v6:Vec<DataFieldT>= Vec::new();
        v6.push(DataFieldT::number(DataFields::PrevailTorqueCompensateValue as i32, 506, 6, None));
        v6.push(DataFieldT::new3(DataFields::TighteningErrorStatus2 as i32, 514, 10, '0', Some(PaddingOrientation::LeftPadded), None));
        hm.insert(6, v6);
        
        let mut v7:Vec<DataFieldT>= Vec::new();
        v7.push(DataFieldT::number(DataFields::CompensatedAngle as i32, 526, 7, None));
        v7.push(DataFieldT::number(DataFields::FinalAngleDecimal as i32, 535, 7, None));
        hm.insert(7, v7);

        let mut v8:Vec<DataFieldT>= Vec::new();
        v8.push(DataFieldT::number(DataFields::StartFinalAngle as i32, 544, 6, None));
        v8.push(DataFieldT::number(DataFields::PostViewTorqueActivated as i32, 552, 1, None));
        v8.push(DataFieldT::number(DataFields::PostViewTorqueHigh as i32, 555, 6, None));
        v8.push(DataFieldT::number(DataFields::PostViewTorqueLow as i32, 563, 6, None));
        hm.insert(8, v8);

        let mut v9:Vec<DataFieldT>= Vec::new();
        v9.push(DataFieldT::number(DataFields::CurrentMonitoringAmp as i32, 571, 5, None));
        v9.push(DataFieldT::number(DataFields::CurrentMonitoringAmpMin as i32, 578, 5, None));
        v9.push(DataFieldT::number(DataFields::CurrentMonitoringAmpMax as i32, 585, 5, None));
        hm.insert(9, v9);

        let mut v10:Vec<DataFieldT>= Vec::new();
        v10.push(DataFieldT::number(DataFields::AngleNumeratorScaleFactor as i32, 592, 5, None));
        v10.push(DataFieldT::number(DataFields::AngleDenominatorScaleFactor as i32, 599, 5, None));
        v10.push(DataFieldT::number(DataFields::OverallAngleStatus as i32, 606, 1, None));
        v10.push(DataFieldT::number(DataFields::OverallAngleMin as i32, 609, 5, None));
        v10.push(DataFieldT::number(DataFields::OverallAngleMax as i32, 616, 5, None));
        v10.push(DataFieldT::number(DataFields::OverallAngle as i32, 623, 5, None));
        v10.push(DataFieldT::number(DataFields::PeakTorque as i32, 630, 6, None));
        v10.push(DataFieldT::number(DataFields::ResidualBreakawayTorque as i32, 638, 6, None));
        v10.push(DataFieldT::number(DataFields::StartRundownAngle as i32, 646, 6, None));
        v10.push(DataFieldT::number(DataFields::RundownAngleComplete as i32, 654, 6, None));
        hm.insert(10, v10);

        let mut v11:Vec<DataFieldT>= Vec::new();
        v11.push(DataFieldT::number(DataFields::ClickTorque as i32, 662, 6, None));
        v11.push(DataFieldT::number(DataFields::ClickAngle as i32, 670, 5, None));
        hm.insert(11, v11);

        let mut v12:Vec<DataFieldT>= Vec::new();
        v12.push(DataFieldT::number(DataFields::NumberOfStagesInMultistage as i32, 526, 2, None));
        v12.push(DataFieldT::number(DataFields::NumberOfStageResults as i32, 530, 2, None));
        v12.push(DataFieldT::new(DataFields::StageResult as i32, 534, 11, None));
        hm.insert(998, v12);

        let mut v13:Vec<DataFieldT>= Vec::new();
        v13.push(DataFieldT::string2(DataFields::VinNumber as i32, 20, 25, Some(false)));
        v13.push(DataFieldT::number(DataFields::JobId as i32, 45, 2, Some(false)));
        v13.push(DataFieldT::number(DataFields::ParameterSetId as i32, 47, 3, Some(false)));
        v13.push(DataFieldT::number(DataFields::BatchSize as i32, 50, 4, Some(false)));
        v13.push(DataFieldT::number(DataFields::BatchCounter as i32, 54, 4, Some(false)));
        v13.push(DataFieldT::number(DataFields::BatchStatus as i32, 58, 1, Some(false)));
        v13.push(DataFieldT::number(DataFields::TighteningStatus as i32, 59, 1, Some(false)));
        v13.push(DataFieldT::number(DataFields::TorqueStatus as i32, 60, 1, Some(false)));
        v13.push(DataFieldT::number(DataFields::AngleStatus as i32, 61, 1, Some(false)));
        v13.push(DataFieldT::number(DataFields::Torque as i32, 62, 6, Some(false)));
        v13.push(DataFieldT::number(DataFields::Angle as i32, 68, 5, Some(false)));
        v13.push(DataFieldT::timestamp(DataFields::Timestamp as i32, 73, Some(false)));
        v13.push(DataFieldT::timestamp(DataFields::LastChangeInParameterSet as i32, 92, Some(false)));
        v13.push(DataFieldT::number(DataFields::TighteningId as i32, 111, 10, Some(false)));
        hm.insert(999, v13);

        hm
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