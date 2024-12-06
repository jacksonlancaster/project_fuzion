use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;

/// <summary>
/// Represents a Tightening Error Status entity
/// </summary>

#[derive(Clone, Default)]
pub struct TighteningErrorStatusT
{
    //Byte 0
    pub rundown_angle_max_shut_off:bool,
    pub rundown_angle_min_shut_off:bool,
    pub torque_max_shut_off:bool,
    pub angle_max_shut_off:bool,
    pub selftap_torque_max_shut_off:bool,
    pub selftap_torque_min_shut_off:bool,
    pub prevail_torque_max_shut_off:bool,
    pub prevail_torque_min_shut_off:bool,

    //Byte 1
    pub prevail_torque_compensate_overflow:bool,
    pub current_monitoring_max_shut_off:bool,
    pub post_view_torque_min_torque_shut_off:bool,
    pub post_view_torque_max_torque_shut_off:bool,
    pub post_view_torque_angle_too_small:bool,
    pub trigger_lost:bool,
    pub torque_less_than_target:bool,
    pub tool_hot:bool,

    //Byte 2
    pub multistage_abort:bool,
    pub rehit:bool,
    pub ds_measure_failed:bool,
    pub current_limit_reached:bool,
    pub end_time_out_shut_off:bool,
    pub remove_fastener_limit_exceeded:bool,
    pub disable_drive:bool,
    pub transducer_lost:bool,

    //Byte 3
    pub transducer_shorted:bool,
    pub transducer_corrupt:bool,
    pub sync_timeout:bool,
    pub dynamic_current_monitoring_min:bool,
    pub dynamic_current_monitoring_max:bool,
    pub angle_max_monitor:bool,
    pub yield_nut_off:bool,
    pub yield_too_few_samples:bool,
}

#[derive(Clone, Default)]
pub struct TighteningErrorStatus2T
{
    pub drive_deactivated:bool,
    pub tool_stall:bool,
    pub drive_hot:bool,
    pub gradient_monitoring_high:bool,
    pub gradient_monitoring_low:bool,
    pub reaction_bar_failed:bool,
    pub snug_max:bool,
    pub cycle_abort:bool,
    pub necking_failure:bool,
    pub effective_loosening:bool,
    pub over_speed:bool,
    pub no_residual_torque:bool,
    pub positioning_fail:bool,
    pub snug_mon_low:bool,
    pub snug_mon_high:bool,
    pub dynamic_min_current:bool,
    pub dynamic_max_current:bool,
    pub latent_result:bool,

    //Bit 19-31
    pub reserved:[u8; 10], //Vec<u8>,
}

impl TighteningErrorStatusT {
    pub fn pack(&mut self)->String
    {
        let bytes = self.pack_bytes();
        std::str::from_utf8(&bytes).expect("Invalid ASCII").to_string()
    }

    pub fn pack_bytes(&mut self)->Vec<u8> {

        let mut bytes:[u8;10]=[0;10];
        bytes[0] = OpenProtocolConvertT::bool_to_byte(vec!
        [
            self.rundown_angle_max_shut_off,
            self.rundown_angle_min_shut_off,
            self.torque_max_shut_off,
            self.angle_max_shut_off,
            self.selftap_torque_max_shut_off,
            self.selftap_torque_min_shut_off,
            self.prevail_torque_max_shut_off,
            self.prevail_torque_min_shut_off
        ]);
        bytes[1] = OpenProtocolConvertT::bool_to_byte(vec!
        [
                self.prevail_torque_compensate_overflow ,
                self.current_monitoring_max_shut_off,
                self.post_view_torque_min_torque_shut_off,
                self.post_view_torque_max_torque_shut_off,
                self.post_view_torque_angle_too_small,
                self.trigger_lost,
                self.torque_less_than_target,
                self.tool_hot
        ]);
        bytes[2] = OpenProtocolConvertT::bool_to_byte(vec!
        [
            self.multistage_abort,
            self.rehit,
            self.ds_measure_failed,
            self.current_limit_reached,
            self.end_time_out_shut_off,
            self.remove_fastener_limit_exceeded,
            self.disable_drive,
            self.transducer_lost
        ]);
        bytes[3] = OpenProtocolConvertT::bool_to_byte(vec!
        [
            self.transducer_shorted,
            self.transducer_corrupt,
            self.sync_timeout,
            self.dynamic_current_monitoring_min,
            self.dynamic_current_monitoring_max,
            self.angle_max_monitor,
            self.yield_nut_off,
            self.yield_too_few_samples
        ]);

        let int_value = i64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let ascii_int = format!("{:010}", int_value);
        ascii_int.into_bytes()

    }

    pub fn  parse_from_str(value:String)->TighteningErrorStatusT
    {
        let long_value = OpenProtocolConvertT::string_to_int64(value);
        let bytes = long_value.to_le_bytes().to_vec();
        
        Self::parse_from_bytes(bytes)
    }

    pub fn  parse_from_bytes(value:Vec<u8>)->TighteningErrorStatusT
    {
        TighteningErrorStatusT
        {
            //byte 0
            rundown_angle_max_shut_off: OpenProtocolConvertT::get_bit(value[0], 1),
            rundown_angle_min_shut_off: OpenProtocolConvertT::get_bit(value[0], 2),
            torque_max_shut_off: OpenProtocolConvertT::get_bit(value[0], 3),
            angle_max_shut_off: OpenProtocolConvertT::get_bit(value[0], 4),
            selftap_torque_max_shut_off: OpenProtocolConvertT::get_bit(value[0], 5),
            selftap_torque_min_shut_off: OpenProtocolConvertT::get_bit(value[0], 6),
            prevail_torque_max_shut_off: OpenProtocolConvertT::get_bit(value[0], 7),
            prevail_torque_min_shut_off: OpenProtocolConvertT::get_bit(value[0], 8),

            //byte 1
            prevail_torque_compensate_overflow: OpenProtocolConvertT::get_bit(value[1], 1),
            current_monitoring_max_shut_off: OpenProtocolConvertT::get_bit(value[1], 2),
            post_view_torque_min_torque_shut_off: OpenProtocolConvertT::get_bit(value[1], 3),
            post_view_torque_max_torque_shut_off: OpenProtocolConvertT::get_bit(value[1], 4),
            post_view_torque_angle_too_small: OpenProtocolConvertT::get_bit(value[1], 5),
            trigger_lost: OpenProtocolConvertT::get_bit(value[1], 6),
            torque_less_than_target: OpenProtocolConvertT::get_bit(value[1], 7),
            tool_hot: OpenProtocolConvertT::get_bit(value[1], 8),

            //byte 2
            multistage_abort: OpenProtocolConvertT::get_bit(value[2], 1),
            rehit: OpenProtocolConvertT::get_bit(value[2], 2),
            ds_measure_failed: OpenProtocolConvertT::get_bit(value[2], 3),
            current_limit_reached: OpenProtocolConvertT::get_bit(value[2], 4),
            end_time_out_shut_off: OpenProtocolConvertT::get_bit(value[2], 5),
            remove_fastener_limit_exceeded: OpenProtocolConvertT::get_bit(value[2], 6),
            disable_drive: OpenProtocolConvertT::get_bit(value[2], 7),
            transducer_lost: OpenProtocolConvertT::get_bit(value[2], 8),

            //byte 3
            transducer_shorted: OpenProtocolConvertT::get_bit(value[3], 1),
            transducer_corrupt: OpenProtocolConvertT::get_bit(value[3], 2),
            sync_timeout: OpenProtocolConvertT::get_bit(value[3], 3),
            dynamic_current_monitoring_min: OpenProtocolConvertT::get_bit(value[3], 4),
            dynamic_current_monitoring_max: OpenProtocolConvertT::get_bit(value[3], 5),
            angle_max_monitor: OpenProtocolConvertT::get_bit(value[3], 6),
            yield_nut_off: OpenProtocolConvertT::get_bit(value[3], 7),
            yield_too_few_samples: OpenProtocolConvertT::get_bit(value[3], 8)
        }
    }
}

impl TighteningErrorStatus2T {
    pub fn new()->TighteningErrorStatus2T
    {
        TighteningErrorStatus2T{..Default::default()}
    }

    pub fn pack(&mut self)->String {
        let bytes = self.pack_bytes();
        std::str::from_utf8(&bytes).expect("Invalid ASCII").to_string()
    }

    pub fn pack_bytes(&mut self)->Vec<u8> {
        let bytes:Vec<u8> =
        vec![
            OpenProtocolConvertT::bool_to_byte(
            vec![
                self.drive_deactivated,
                self.tool_stall,
                self.drive_hot,
                self.gradient_monitoring_high,
                self.gradient_monitoring_low,
                self.reaction_bar_failed,
                self.snug_max,
                self.cycle_abort,
            ]),
            OpenProtocolConvertT::bool_to_byte(
            vec![
                self.necking_failure,
                self.effective_loosening,
                self.over_speed,
                self.no_residual_torque,
                self.positioning_fail,
                self.snug_mon_low,
                self.snug_mon_high,
                self.dynamic_min_current,
            ]),
            OpenProtocolConvertT::bool_to_byte(
            vec![
                self.dynamic_max_current,
                self.latent_result,
                OpenProtocolConvertT::get_bit(self.reserved[2], 3),
                OpenProtocolConvertT::get_bit(self.reserved[2], 4),
                OpenProtocolConvertT::get_bit(self.reserved[2], 5),
                OpenProtocolConvertT::get_bit(self.reserved[2], 6),
                OpenProtocolConvertT::get_bit(self.reserved[2], 7),
                OpenProtocolConvertT::get_bit(self.reserved[2], 8)
            ]),
            self.reserved[3],
            self.reserved[4],
            self.reserved[5],
            self.reserved[6],
            self.reserved[7],
            self.reserved[8],
            self.reserved[9]
        ];

        let int_value = i64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let ascii_int = format!("{:010}", int_value);
        ascii_int.into_bytes()
    }

    pub fn parse_from_str(value:String)->TighteningErrorStatus2T
    {
        let longValue = OpenProtocolConvertT::string_to_int64(value);
        let bytes = longValue.to_le_bytes().to_vec();
        Self::parse_from_bytes(bytes)
    }

    pub fn parse_from_bytes(value:Vec<u8>)->TighteningErrorStatus2T
    {
        let mut obj = TighteningErrorStatus2T
        {
            //byte 0
            drive_deactivated: OpenProtocolConvertT::get_bit(value[0], 1),
            tool_stall: OpenProtocolConvertT::get_bit(value[0], 2),
            drive_hot: OpenProtocolConvertT::get_bit(value[0], 3),
            gradient_monitoring_high: OpenProtocolConvertT::get_bit(value[0], 4),
            gradient_monitoring_low: OpenProtocolConvertT::get_bit(value[0], 5),
            reaction_bar_failed: OpenProtocolConvertT::get_bit(value[0], 6),
            snug_max: OpenProtocolConvertT::get_bit(value[0], 7),
            cycle_abort: OpenProtocolConvertT::get_bit(value[0], 8),

            //byte 1
            necking_failure: OpenProtocolConvertT::get_bit(value[1], 1),
            effective_loosening: OpenProtocolConvertT::get_bit(value[1], 2),
            over_speed: OpenProtocolConvertT::get_bit(value[1], 3),
            no_residual_torque: OpenProtocolConvertT::get_bit(value[1], 4),
            positioning_fail: OpenProtocolConvertT::get_bit(value[1], 5),
            snug_mon_low: OpenProtocolConvertT::get_bit(value[1], 6),
            snug_mon_high: OpenProtocolConvertT::get_bit(value[1], 7),
            dynamic_min_current: OpenProtocolConvertT::get_bit(value[1], 8),

            //byte 2
            dynamic_max_current: OpenProtocolConvertT::get_bit(value[2], 1),
            latent_result: OpenProtocolConvertT::get_bit(value[2], 2),
            reserved:Default::default()
        };

        //set only 19 and 20 bytes to reserved
        obj.reserved[0] = OpenProtocolConvertT::bool_to_byte(vec![OpenProtocolConvertT::get_bit(value[2], 3), OpenProtocolConvertT::get_bit(value[2], 4), false, false, false, false, false, false]);

        return obj;
    }
}