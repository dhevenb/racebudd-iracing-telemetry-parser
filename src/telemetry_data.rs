use crate::utils::{read_file_bytes, ReadSeek};
use crate::headers::VarInfo;

#[derive(Debug)]
pub enum ChannelValue {
    Char(char),
    Bool(bool),
    Int(i32),
    BitField(u32),
    Float32(f32),
    Float64(f64),
}

impl ChannelValue {
    pub fn char(&self) -> char {
        if let ChannelValue::Char(x) = self {
            *x
        } else {
            panic!()
        }
    }

    pub fn bool(&self) -> bool {
        if let ChannelValue::Bool(x) = self {
            *x
        } else {
            panic!()
        }
    }

    pub fn float_32(&self) -> f32 {
        if let ChannelValue::Float32(x) = self {
            *x
        } else {
            panic!()
        }
    }

    pub fn float_64(&self) -> f64 {
        if let ChannelValue::Float64(x) = self {
            *x
        } else {
            panic!()
        }
    }

    pub fn int(&self) -> i32 {
        if let ChannelValue::Int(x) = self {
            *x
        } else {
            panic!()
        }
    }

    pub fn bitfield(&self) -> u32 {
        if let ChannelValue::BitField(x) = self {
            *x
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
pub struct Ticks<'a> {
    pub file: &'a mut Box<dyn ReadSeek>,
    pub channels: Vec<VarInfo>,
    pub tick_length: i32,
    pub buf_offset: i32,
    pub tick_number: i32,
}

impl<'a> Iterator for Ticks<'a> {
    type Item = Tick;

    fn next(&mut self) -> Option<Self::Item> {
        let tick_start_pos = self.buf_offset + (self.tick_number * self.tick_length);

        match read_file_bytes(self.file, tick_start_pos as usize, self.tick_length as usize) {
            Ok(bytes) => {
                self.tick_number += 1;
                let channels = self.channels.clone();

                Some(Tick {
                    bytes,
                    channels,
                })
            },
            Err(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tick {
    bytes: Vec<u8>,
    channels: Vec<VarInfo>,
}

macro_rules! generate_getters {
    // maps function name to iRacing variable name
    ($($name:ident => $iracing_var:expr),* $(,)?) => {
        $(
            pub fn $name(&self) -> ChannelValue {
                let var = self.channels.iter()
                    .find(|var| var.name == $iracing_var)
                    .unwrap();
                self.get_value(var).unwrap()
            }
        )*
    };
}

impl Tick {

    pub fn get_value(&self, var: &VarInfo) -> Option<ChannelValue> {
        let offset = var.offset as usize;
        match var.var_type {
            0 => {
                let size = 1;
                Some(ChannelValue::Char(self.bytes[offset + size] as char))
            }
            1 => {
                let size = 1;
                Some(ChannelValue::Bool(self.bytes[offset + size] != 0))
            }
            2 => {
                let size = 4;
                let value =
                    i32::from_le_bytes(self.bytes[offset..(offset + size)].try_into().unwrap());
                Some(ChannelValue::Int(value))
            }
            3 => {
                let size = 4;
                let value =
                    u32::from_le_bytes(self.bytes[offset..(offset + size)].try_into().unwrap());
                Some(ChannelValue::BitField(value))
            }
            4 => {
                let size = 4;
                let value =
                    f32::from_le_bytes(self.bytes[offset..(offset + size)].try_into().unwrap());
                Some(ChannelValue::Float32(value))
            }
            5 => {
                let size = 8;
                let value =
                    f64::from_le_bytes(self.bytes[offset..(offset + size)].try_into().unwrap());
                Some(ChannelValue::Float64(value))
            }
            _ => unimplemented!(),
        }
    }

    generate_getters! {
        // Session Information
        session_time => "SessionTime",
        session_tick => "SessionTick",
        session_num => "SessionNum",
        session_state => "SessionState",
        session_unique_id => "SessionUniqueID",
        session_flags => "SessionFlags",
        session_time_remain => "SessionTimeRemain",
        session_laps_remain => "SessionLapsRemain",
        session_laps_remain_ex => "SessionLapsRemainEx",
        session_time_total => "SessionTimeTotal",
        session_laps_total => "SessionLapsTotal",
        session_joker_laps_remain => "SessionJokerLapsRemain",
        session_on_joker_lap => "SessionOnJokerLap",
        session_time_of_day => "SessionTimeOfDay",
        driver_marker => "DriverMarker",
        push_to_talk => "PushToTalk",
        push_to_pass => "PushToPass",
        manual_boost => "ManualBoost",
        manual_no_boost => "ManualNoBoost",
        is_on_track => "IsOnTrack",
        frame_rate => "FrameRate",
        cpu_usage_fg => "CpuUsageFG",
        gpu_usage => "GpuUsage",
        chan_avg_latency => "ChanAvgLatency",
        chan_latency => "ChanLatency",
        chan_quality => "ChanQuality",
        chan_partner_quality => "ChanPartnerQuality",
        cpu_usage_bg => "CpuUsageBG",
        chan_clock_skew => "ChanClockSkew",
        mem_page_fault_sec => "MemPageFaultSec",
        mem_soft_page_fault_sec => "MemSoftPageFaultSec",
        player_car_position => "PlayerCarPosition",
        player_car_class_position => "PlayerCarClassPosition",
        player_car_class => "PlayerCarClass",
        player_track_surface => "PlayerTrackSurface",
        player_track_surface_material => "PlayerTrackSurfaceMaterial",
        player_car_idx => "PlayerCarIdx",
        player_car_team_incident_count => "PlayerCarTeamIncidentCount",
        player_car_my_incident_count => "PlayerCarMyIncidentCount",
        player_car_driver_incident_count => "PlayerCarDriverIncidentCount",
        player_car_weight_penalty => "PlayerCarWeightPenalty",
        player_car_power_adjust => "PlayerCarPowerAdjust",
        player_car_dry_tire_set_limit => "PlayerCarDryTireSetLimit",
        player_car_tow_time => "PlayerCarTowTime",
        player_car_in_pit_stall => "PlayerCarInPitStall",
        player_car_pit_sv_status => "PlayerCarPitSvStatus",
        player_tire_compound => "PlayerTireCompound",
        player_fast_repairs_used => "PlayerFastRepairsUsed",
        pace_mode => "PaceMode",
        on_pit_road => "OnPitRoad",
        steering_wheel_angle => "SteeringWheelAngle",
        throttle => "Throttle",
        brake => "Brake",
        clutch => "Clutch",
        gear => "Gear",
        rpm => "RPM",
        lap => "Lap",
        lap_completed => "LapCompleted",
        lap_dist => "LapDist",
        lap_dist_pct => "LapDistPct",
        car_dist_ahead => "CarDistAhead",
        car_dist_behind => "CarDistBehind",
        lap_best_lap => "LapBestLap",
        lap_best_lap_time => "LapBestLapTime",
        lap_last_lap_time => "LapLastLapTime",
        lap_current_lap_time => "LapCurrentLapTime",
        lap_las_n_lap_seq => "LapLasNLapSeq",
        lap_last_n_lap_time => "LapLastNLapTime",
        lap_best_n_lap_lap => "LapBestNLapLap",
        lap_best_n_lap_time => "LapBestNLapTime",
        lap_delta_to_best_lap => "LapDeltaToBestLap",
        lap_delta_to_best_lap_dd => "LapDeltaToBestLap_DD",
        lap_delta_to_best_lap_ok => "LapDeltaToBestLap_OK",
        lap_delta_to_optimal_lap => "LapDeltaToOptimalLap",
        lap_delta_to_optimal_lap_dd => "LapDeltaToOptimalLap_DD",
        lap_delta_to_optimal_lap_ok => "LapDeltaToOptimalLap_OK",
        lap_delta_to_session_best_lap => "LapDeltaToSessionBestLap",
        lap_delta_to_session_best_lap_dd => "LapDeltaToSessionBestLap_DD",
        lap_delta_to_session_best_lap_ok => "LapDeltaToSessionBestLap_OK",
        lap_delta_to_session_optimal_lap => "LapDeltaToSessionOptimalLap",
        lap_delta_to_session_optimal_lap_dd => "LapDeltaToSessionOptimalLap_DD",
        lap_delta_to_session_optimal_lap_ok => "LapDeltaToSessionOptimalLap_OK",
        lap_delta_to_session_lastl_lap => "LapDeltaToSessionLastlLap",
        lap_delta_to_session_lastl_lap_dd => "LapDeltaToSessionLastlLap_DD",
        lap_delta_to_session_lastl_lap_ok => "LapDeltaToSessionLastlLap_OK",
        speed => "Speed",
        yaw => "Yaw",
        yaw_north => "YawNorth",
        pitch => "Pitch",
        roll => "Roll",
        enter_exit_reset => "EnterExitReset",
        lat => "Lat",
        lon => "Lon",
        alt => "Alt",
        track_temp => "TrackTemp",
        track_temp_crew => "TrackTempCrew",
        air_temp => "AirTemp",
        track_wetness => "TrackWetness",
        skies => "Skies",
        air_density => "AirDensity",
        air_pressure => "AirPressure",
        wind_vel => "WindVel",
        wind_dir => "WindDir",
        relative_humidity => "RelativeHumidity",
        fog_level => "FogLevel",
        precipitation => "Precipitation",
        solar_altitude => "SolarAltitude",
        solar_azimuth => "SolarAzimuth",
        weather_declared_wet => "WeatherDeclaredWet",
        pits_open => "PitsOpen",
        pit_repair_left => "PitRepairLeft",
        pit_opt_repair_left => "PitOptRepairLeft",
        pitstop_active => "PitstopActive",
        fast_repair_used => "FastRepairUsed",
        fast_repair_available => "FastRepairAvailable",
        lf_tires_used => "LFTiresUsed",
        rf_tires_used => "RFTiresUsed",
        lr_tires_used => "LRTiresUsed",
        rr_tires_used => "RRTiresUsed",
        left_tire_sets_used => "LeftTireSetsUsed",
        right_tire_sets_used => "RightTireSetsUsed",
        front_tire_sets_used => "FrontTireSetsUsed",
        rear_tire_sets_used => "RearTireSetsUsed",
        tire_sets_used => "TireSetsUsed",
        lf_tires_available => "LFTiresAvailable",
        rf_tires_available => "RFTiresAvailable",
        lr_tires_available => "LRTiresAvailable",
        rr_tires_available => "RRTiresAvailable",
        left_tire_sets_available => "LeftTireSetsAvailable",
        right_tire_sets_available => "RightTireSetsAvailable",
        front_tire_sets_available => "FrontTireSetsAvailable",
        rear_tire_sets_available => "RearTireSetsAvailable",
        tire_sets_available => "TireSetsAvailable",
        is_on_track_car => "IsOnTrackCar",
        steering_wheel_angle_max => "SteeringWheelAngleMax",
        shift_power_pct => "ShiftPowerPct",
        shift_grind_rpm => "ShiftGrindRPM",
        throttle_raw => "ThrottleRaw",
        brake_raw => "BrakeRaw",
        clutch_raw => "ClutchRaw",
        handbrake_raw => "HandbrakeRaw",
        brake_abs_active => "BrakeABSactive",
        brake_abs_cut_pct => "BrakeABScutPct",
        engine_warnings => "EngineWarnings",
        fuel_level_pct => "FuelLevelPct",
        pit_sv_flags => "PitSvFlags",
        pit_sv_lfp => "PitSvLFP",
        pit_sv_rfp => "PitSvRFP",
        pit_sv_lrp => "PitSvLRP",
        pit_sv_rrp => "PitSvRRP",
        pit_sv_fuel => "PitSvFuel",
        pit_sv_tire_compound => "PitSvTireCompound",
        p2p_status => "P2P_Status",
        p2p_count => "P2P_Count",
        steering_wheel_pct_torque => "SteeringWheelPctTorque",
        steering_wheel_pct_torque_sign => "SteeringWheelPctTorqueSign",
        steering_wheel_pct_torque_sign_stops => "SteeringWheelPctTorqueSignStops",
        steering_wheel_pct_intensity => "SteeringWheelPctIntensity",
        steering_wheel_pct_smoothing => "SteeringWheelPctSmoothing",
        steering_wheel_pct_damper => "SteeringWheelPctDamper",
        steering_wheel_limiter => "SteeringWheelLimiter",
        steering_wheel_max_force_nm => "SteeringWheelMaxForceNm",
        steering_wheel_use_linear => "SteeringWheelUseLinear",
        shift_indicator_pct => "ShiftIndicatorPct",
        tire_lf_rumble_pitch => "TireLF_RumblePitch",
        tire_rf_rumble_pitch => "TireRF_RumblePitch",
        tire_lr_rumble_pitch => "TireLR_RumblePitch",
        tire_rr_rumble_pitch => "TireRR_RumblePitch",
        steering_wheel_torque_st => "SteeringWheelTorque_ST",
        steering_wheel_torque => "SteeringWheelTorque",
        velocity_z => "VelocityZ",
        velocity_y => "VelocityY",
        velocity_x => "VelocityX",
        yaw_rate => "YawRate",
        pitch_rate => "PitchRate",
        roll_rate => "RollRate",
        vert_accel => "VertAccel",
        lat_accel => "LatAccel",
        long_accel => "LongAccel",
        dc_starter => "dcStarter",
        dc_traction_control_toggle => "dcTractionControlToggle",
        dc_pit_speed_limiter_toggle => "dcPitSpeedLimiterToggle",
        dc_headlight_flash => "dcHeadlightFlash",
        dp_rf_tire_change => "dpRFTireChange",
        dp_lf_tire_change => "dpLFTireChange",
        dp_rr_tire_change => "dpRRTireChange",
        dp_lr_tire_change => "dpLRTireChange",
        dp_fuel_fill => "dpFuelFill",
        dp_windshield_tearoff => "dpWindshieldTearoff",
        dp_fuel_add_kg => "dpFuelAddKg",
        dp_fast_repair => "dpFastRepair",
        dc_brake_bias => "dcBrakeBias",
        dp_lf_tire_cold_press => "dpLFTireColdPress",
        dp_rf_tire_cold_press => "dpRFTireColdPress",
        dp_lr_tire_cold_press => "dpLRTireColdPress",
        dp_rr_tire_cold_press => "dpRRTireColdPress",
        dc_dash_page => "dcDashPage",
        dc_traction_control => "dcTractionControl",
        dc_abs => "dcABS",
        dp_fuel_auto_fill_enabled => "dpFuelAutoFillEnabled",
        dp_fuel_auto_fill_active => "dpFuelAutoFillActive",
        dc_toggle_windshield_wipers => "dcToggleWindshieldWipers",
        dc_trigger_windshield_wipers => "dcTriggerWindshieldWipers",
        fuel_use_per_hour => "FuelUsePerHour",
        voltage => "Voltage",
        water_temp => "WaterTemp",
        water_level => "WaterLevel",
        fuel_press => "FuelPress",
        oil_temp => "OilTemp",
        oil_press => "OilPress",
        oil_level => "OilLevel",
        manifold_press => "ManifoldPress",
        fuel_level => "FuelLevel",
        engine_0_rpm => "Engine0_RPM",
        rf_brake_line_press => "RFbrakeLinePress",
        rf_speed => "RFspeed",
        rf_pressure => "RFpressure",
        rf_cold_pressure => "RFcoldPressure",
        rf_temp_l => "RFtempL",
        rf_temp_m => "RFtempM",
        rf_temp_r => "RFtempR",
        rf_temp_cl => "RFtempCL",
        rf_temp_cm => "RFtempCM",
        rf_temp_cr => "RFtempCR",
        rf_wear_l => "RFwearL",
        rf_wear_m => "RFwearM",
        rf_wear_r => "RFwearR",
        lf_brake_line_press => "LFbrakeLinePress",
        lf_speed => "LFspeed",
        lf_pressure => "LFpressure",
        lf_cold_pressure => "LFcoldPressure",
        lf_temp_l => "LFtempL",
        lf_temp_m => "LFtempM",
        lf_temp_r => "LFtempR",
        lf_temp_cl => "LFtempCL",
        lf_temp_cm => "LFtempCM",
        lf_temp_cr => "LFtempCR",
        lf_wear_l => "LFwearL",
        lf_wear_m => "LFwearM",
        lf_wear_r => "LFwearR",
        rr_brake_line_press => "RRbrakeLinePress",
        rr_speed => "RRspeed",
        rr_pressure => "RRpressure",
        rr_cold_pressure => "RRcoldPressure",
        rr_temp_l => "RRtempL",
        rr_temp_m => "RRtempM",
        rr_temp_r => "RRtempR",
        rr_temp_cl => "RRtempCL",
        rr_temp_cm => "RRtempCM",
        rr_temp_cr => "RRtempCR",
        rr_wear_l => "RRwearL",
        rr_wear_m => "RRwearM",
        rr_wear_r => "RRwearR",
        lr_brake_line_press => "LRbrakeLinePress",
        lr_speed => "LRspeed",
        lr_pressure => "LRpressure",
        lr_cold_pressure => "LRcoldPressure",
        lr_temp_l => "LRtempL",
        lr_temp_m => "LRtempM",
        lr_temp_r => "LRtempR",
        lr_temp_cr => "LRtempCR",
        lr_wear_l => "LRwearL",
        lr_wear_m => "LRwearM",
        lr_shock_defl => "LRshockDefl",
        lr_shock_vel => "LRshockVel",
        rr_shock_defl => "RRshockDefl",
        rr_shock_vel => "RRshockVel",
        lf_shock_defl => "LFshockDefl",
        lf_shock_vel => "LFshockVel",
        rf_shock_defl => "RFshockDefl",
        rf_shock_vel => "RFshockVel",
        lf_ride_height => "LFrideHeight",
        rf_ride_height => "RFrideHeight",
        lr_ride_height => "LRrideHeight",
        rr_ride_height => "RRrideHeight",
        cfsr_ride_height => "CFSRrideHeight"
    }
}