use crate::OpenProtocolInterpreter::OpenProtocolConvert::OpenProtocolConvertT;

/// <summary>
/// Represents a Tightening Error Status entity
/// </summary>

#[derive(Clone)]
pub struct TighteningErrorStatusT
{
    //Byte 0
    pub RundownAngleMaxShutOff:bool,
    pub RundownAngleMinShutOff:bool,
    pub TorqueMaxShutOff:bool,
    pub AngleMaxShutOff:bool,
    pub SelftapTorqueMaxShutOff:bool,
    pub SelftapTorqueMinShutOff:bool,
    pub PrevailTorqueMaxShutOff:bool,
    pub PrevailTorqueMinShutOff:bool,

    //Byte 1
    pub PrevailTorqueCompensateOverflow:bool,
    pub CurrentMonitoringMaxShutOff:bool,
    pub PostViewTorqueMinTorqueShutOff:bool,
    pub PostViewTorqueMaxTorqueShutOff:bool,
    pub PostViewTorqueAngleTooSmall:bool,
    pub TriggerLost:bool,
    pub TorqueLessThanTarget:bool,
    pub ToolHot:bool,

    //Byte 2
    pub MultistageAbort:bool,
    pub Rehit:bool,
    pub DsMeasureFailed:bool,
    pub CurrentLimitReached:bool,
    pub EndTimeOutShutOff:bool,
    pub RemoveFastenerLimitExceeded:bool,
    pub DisableDrive:bool,
    pub TransducerLost:bool,

    //Byte 3
    pub TransducerShorted:bool,
    pub TransducerCorrupt:bool,
    pub SyncTimeout:bool,
    pub DynamicCurrentMonitoringMin:bool,
    pub DynamicCurrentMonitoringMax:bool,
    pub AngleMaxMonitor:bool,
    pub YieldNutOff:bool,
    pub YieldTooFewSamples:bool,
}

#[derive(Clone)]
pub struct TighteningErrorStatus2T
{
    pub DriveDeactivated:bool,
    pub ToolStall:bool,
    pub DriveHot:bool,
    pub GradientMonitoringHigh:bool,
    pub GradientMonitoringLow:bool,
    pub ReactionBarFailed:bool,
    pub SnugMax:bool,
    pub CycleAbort:bool,
    pub NeckingFailure:bool,
    pub EffectiveLoosening:bool,
    pub OverSpeed:bool,
    pub NoResidualTorque:bool,
    pub PositioningFail:bool,
    pub SnugMonLow:bool,
    pub SnugMonHigh:bool,
    pub DynamicMinCurrent:bool,
    pub DynamicMaxCurrent:bool,
    pub LatentResult:bool,

    //Bit 19-31
    pub Reserved:Vec<u8>,
}

impl TighteningErrorStatusT {
    pub fn Pack(&mut self)->String
    {
        byte[] bytes = PackBytes();
        return Encoding.ASCII.GetString(bytes);
    }

    pub fn PackBytes(&mut self)->Vec<u8> {

        let mut bytes:[u8;10]=[0;10];
        bytes[0] = OpenProtocolConvertT::bool_to_byte(vec!
        [
            self.RundownAngleMaxShutOff,
            self.RundownAngleMinShutOff,
            self.TorqueMaxShutOff,
            self.AngleMaxShutOff,
            self.SelftapTorqueMaxShutOff,
            self.SelftapTorqueMinShutOff,
            self.PrevailTorqueMaxShutOff,
            self.PrevailTorqueMinShutOff
        ]);
        bytes[1] = OpenProtocolConvertT::bool_to_byte(vec!
        [
                self.PrevailTorqueCompensateOverflow ,
                self.CurrentMonitoringMaxShutOff,
                self.PostViewTorqueMinTorqueShutOff,
                self.PostViewTorqueMaxTorqueShutOff,
                self.PostViewTorqueAngleTooSmall,
                self.TriggerLost,
                self.TorqueLessThanTarget,
                self.ToolHot
        ]);
        bytes[2] = OpenProtocolConvertT::bool_to_byte(vec!
        [
            self.MultistageAbort,
            self.Rehit,
            self.DsMeasureFailed,
            self.CurrentLimitReached,
            self.EndTimeOutShutOff,
            self.RemoveFastenerLimitExceeded,
            self.DisableDrive,
            self.TransducerLost
        ]);
        bytes[3] = OpenProtocolConvertT::bool_to_byte(vec!
        [
            self.TransducerShorted,
            self.TransducerCorrupt,
            self.SyncTimeout,
            self.DynamicCurrentMonitoringMin,
            self.DynamicCurrentMonitoringMax,
            self.AngleMaxMonitor,
            self.YieldNutOff,
            self.YieldTooFewSamples
        ]);

        var asciiLong = System.BitConverter.ToInt64(bytes, 0).ToString("D10");
        return Encoding.ASCII.GetBytes(asciiLong);
    }

    pub fn  parse_from_str(value:String)->TighteningErrorStatusT
    {
        let longValue = OpenProtocolConvertT::string_to_int64(value);
        let bytes = longValue.to_le_bytes().to_vec();
        
        Self::parse_from_bytes(bytes)
    }

    pub fn  parse_from_bytes(value:Vec<u8>)->TighteningErrorStatusT
    {
        TighteningErrorStatusT
        {
            //byte 0
            RundownAngleMaxShutOff: OpenProtocolConvertT::get_bit(value[0], 1),
            RundownAngleMinShutOff: OpenProtocolConvertT::get_bit(value[0], 2),
            TorqueMaxShutOff: OpenProtocolConvertT::get_bit(value[0], 3),
            AngleMaxShutOff: OpenProtocolConvertT::get_bit(value[0], 4),
            SelftapTorqueMaxShutOff: OpenProtocolConvertT::get_bit(value[0], 5),
            SelftapTorqueMinShutOff: OpenProtocolConvertT::get_bit(value[0], 6),
            PrevailTorqueMaxShutOff: OpenProtocolConvertT::get_bit(value[0], 7),
            PrevailTorqueMinShutOff: OpenProtocolConvertT::get_bit(value[0], 8),

            //byte 1
            PrevailTorqueCompensateOverflow: OpenProtocolConvertT::get_bit(value[1], 1),
            CurrentMonitoringMaxShutOff: OpenProtocolConvertT::get_bit(value[1], 2),
            PostViewTorqueMinTorqueShutOff: OpenProtocolConvertT::get_bit(value[1], 3),
            PostViewTorqueMaxTorqueShutOff: OpenProtocolConvertT::get_bit(value[1], 4),
            PostViewTorqueAngleTooSmall: OpenProtocolConvertT::get_bit(value[1], 5),
            TriggerLost: OpenProtocolConvertT::get_bit(value[1], 6),
            TorqueLessThanTarget: OpenProtocolConvertT::get_bit(value[1], 7),
            ToolHot: OpenProtocolConvertT::get_bit(value[1], 8),

            //byte 2
            MultistageAbort: OpenProtocolConvertT::get_bit(value[2], 1),
            Rehit: OpenProtocolConvertT::get_bit(value[2], 2),
            DsMeasureFailed: OpenProtocolConvertT::get_bit(value[2], 3),
            CurrentLimitReached: OpenProtocolConvertT::get_bit(value[2], 4),
            EndTimeOutShutOff: OpenProtocolConvertT::get_bit(value[2], 5),
            RemoveFastenerLimitExceeded: OpenProtocolConvertT::get_bit(value[2], 6),
            DisableDrive: OpenProtocolConvertT::get_bit(value[2], 7),
            TransducerLost: OpenProtocolConvertT::get_bit(value[2], 8),

            //byte 3
            TransducerShorted: OpenProtocolConvertT::get_bit(value[3], 1),
            TransducerCorrupt: OpenProtocolConvertT::get_bit(value[3], 2),
            SyncTimeout: OpenProtocolConvertT::get_bit(value[3], 3),
            DynamicCurrentMonitoringMin: OpenProtocolConvertT::get_bit(value[3], 4),
            DynamicCurrentMonitoringMax: OpenProtocolConvertT::get_bit(value[3], 5),
            AngleMaxMonitor: OpenProtocolConvertT::get_bit(value[3], 6),
            YieldNutOff: OpenProtocolConvertT::get_bit(value[3], 7),
            YieldTooFewSamples: OpenProtocolConvertT::get_bit(value[3], 8)
        }
    }
}

impl TighteningErrorStatus2T {
    public TighteningErrorStatus2()
    {
        Reserved = new byte[10];
    }

    pub fn Pack()->String {
        byte[] bytes = PackBytes();
        return Encoding.ASCII.GetString(bytes);
    }

    pub fn PackBytes()->Vec<u8> {
        byte[] bytes =
        [
            OpenProtocolConvert.ToByte(
            [
                DriveDeactivated,
                ToolStall,
                DriveHot,
                GradientMonitoringHigh,
                GradientMonitoringLow,
                ReactionBarFailed,
                SnugMax,
                CycleAbort,
            ]),
            OpenProtocolConvert.ToByte(
            [
                NeckingFailure,
                EffectiveLoosening,
                OverSpeed,
                NoResidualTorque,
                PositioningFail,
                SnugMonLow,
                SnugMonHigh,
                DynamicMinCurrent,
            ]),
            OpenProtocolConvert.ToByte(
            [
                DynamicMaxCurrent,
                LatentResult,
                OpenProtocolConvertT::get_bit(Reserved[2], 3),
                OpenProtocolConvertT::get_bit(Reserved[2], 4),
                OpenProtocolConvertT::get_bit(Reserved[2], 5),
                OpenProtocolConvertT::get_bit(Reserved[2], 6),
                OpenProtocolConvertT::get_bit(Reserved[2], 7),
                OpenProtocolConvertT::get_bit(Reserved[2], 8)
            ]),
            Reserved[3],
            Reserved[4],
            Reserved[5],
            Reserved[6],
            Reserved[7],
            Reserved[8],
            Reserved[9]
        ];

        var asciiLong = System.BitConverter.ToInt64(bytes, 0).ToString().PadLeft(10, '0');
        return Encoding.ASCII.GetBytes(asciiLong);
    }

    pub fn parse_from_str(value:String)->TighteningErrorStatus2T
    {
        let longValue = OpenProtocolConvertT::string_to_int64(value);
        let bytes = longValue.to_le_bytes().to_vec();
        Self::parse_from_bytes(bytes)
    }

    pub fn parse_from_bytes(value:Vec<u8>)->TighteningErrorStatus2T
    {
        let obj = TighteningErrorStatus2T
        {
            //byte 0
            DriveDeactivated: OpenProtocolConvertT::get_bit(value[0], 1),
            ToolStall: OpenProtocolConvertT::get_bit(value[0], 2),
            DriveHot: OpenProtocolConvertT::get_bit(value[0], 3),
            GradientMonitoringHigh: OpenProtocolConvertT::get_bit(value[0], 4),
            GradientMonitoringLow: OpenProtocolConvertT::get_bit(value[0], 5),
            ReactionBarFailed: OpenProtocolConvertT::get_bit(value[0], 6),
            SnugMax: OpenProtocolConvertT::get_bit(value[0], 7),
            CycleAbort: OpenProtocolConvertT::get_bit(value[0], 8),

            //byte 1
            NeckingFailure: OpenProtocolConvertT::get_bit(value[1], 1),
            EffectiveLoosening: OpenProtocolConvertT::get_bit(value[1], 2),
            OverSpeed: OpenProtocolConvertT::get_bit(value[1], 3),
            NoResidualTorque: OpenProtocolConvertT::get_bit(value[1], 4),
            PositioningFail: OpenProtocolConvertT::get_bit(value[1], 5),
            SnugMonLow: OpenProtocolConvertT::get_bit(value[1], 6),
            SnugMonHigh: OpenProtocolConvertT::get_bit(value[1], 7),
            DynamicMinCurrent: OpenProtocolConvertT::get_bit(value[1], 8),

            //byte 2
            DynamicMaxCurrent: OpenProtocolConvertT::get_bit(value[2], 1),
            LatentResult: OpenProtocolConvertT::get_bit(value[2], 2),
            Reserved:[u8; 10]
        };

        //set only 19 and 20 bytes to reserved
        obj.Reserved[0] = OpenProtocolConvertT::bool_to_byte([OpenProtocolConvertT::get_bit(value[2], 3), OpenProtocolConvertT::get_bit(value[2], 4), false, false, false, false, false, false]);

        return obj;
    }
}