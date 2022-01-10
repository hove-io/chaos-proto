// Copyright  (C) 2021, Kisio Digital and/or its affiliates. All rights reserved.
//
// This file is part of Navitia,
// the software to build cool stuff with public transport.
//
// Hope you'll enjoy and contribute to this project,
// powered by Kisio Digital (www.kisio.com).
// Help us simplify mobility and open public transport:
// a non ending quest to the responsive locomotion way of traveling!
//
//
// LICENCE: This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Stay tuned using
// twitter @navitia
// channel `#navitia` on riot https://riot.im/app/#/room/#navitia:matrix.org
// https://groups.google.com/d/forum/navitia
// www.navitia.io


pub struct TripUpdate {
    trip: TripDescriptor,

    // present only for an added trip
    vehicle: Option<VehicleDescriptor>,

    // This should always contains all stops this vehicle go through.
    // If the vehicle is to be deleted, it should contains the stops of the deleted vehicle.
    // However, the list may be empty for deleted vehicles, and the consumer should handle this case.
    stop_time_updates: Vec<StopTimeUpdate>,

    // never present 
    timestamp: Option<u64>,

    /// kirin extensions

    // sometimes present. To be used for display in responses.
    trip_message: Option<String>, 
    // sometimes present. To be used for display in responses.
    headsign: Option<String>,   

    // always present, used to determine  whether :
    //  - this is a NEW vehicle (not in ntfs)
    //  - this is a modification of an EXISTING vehicle (in ntfs, or a previously received new vehicle)
    //  - this is a vehicle to be deleted.
    // 
    // Please be advised that this effect is *relative to the base-schedule*,
    // and not relative to last information exchanged.
    // So for a trip that is added then delayed, will  have an two updates with the "AdditionnalService" effect.
    //
    // The same goes for deletion (in case details are added later to the deleted trip, like supplementary deleted-stops)
    effect: Option<AlertEffect>,  
}

pub enum AlertEffect {
    NoService,
    ReducedService,
    SignificantDelay,
    Detour,
    AdditionnalService,
    ModifiedService,
    OtherEffect,
    UnknownEffect,
    // never happens for now
    StopMoved,
}

pub struct TripDescriptor {
    // always here
    // == vehicle_journey_id if this concerns a vehicle_journey found in the ntfs 
    trip_id: String,

    // unused for now
    route_id: Option<String>,

    // never present
    start_time: Option<String>,
    // always present, used to identify the UTC day for the start of vehicle_journey 
    // concerned by this disruption
    start_date: Option<String>,

    // field to be removed
    schedule_relationship: Option<TripDescriptorScheduleRelationship>,

    /// kirin extensions
    // not always present. To be used for display in responses.
    contributor: Option<String>,
    // not always present. To be used for display in responses.
    company_id: Option<String>,
}

enum TripDescriptorScheduleRelationship {
    Canceled,
    Added,
    Scheduled,

    // these two below never happen in Kirin, 
    Unscheduled,
    Replacement,
}

pub struct VehicleDescriptor {
    // not always present. To be used for display in responses.
    physical_mode_id: Option<String>,

    // never present,
    id: Option<String>,
    // never present
    label: Option<String>,
    // never present
    licence_plate: Option<String>,
}

pub struct StopTimeUpdate {
    // useless 
    stop_sequence: Option<u32>,

    // always_present,
    // it can be a stop_id not present in NTFS 
    stop_id: Option<String>,

    // For a deleted stop, both arrival and departure can be None.
    // Otherwise, at least one of arrival/departure is present
    arrival: Option<StopTimeEvent>,
    departure: Option<StopTimeEvent>,

    // never present
    schedule_relationship: Option<StopTimeUpdateScheduleRelationship>,

    // To be used for display in responses.
    stoptime_message: Option<String>,
}

pub enum StopTimeUpdateScheduleRelationship {
    Scheduled,
    Skipped,
    Added,
    // never happens for now
    NoData,
}

pub struct StopTimeEvent {
    // useful currently, to determine the actual status at stop in
    // case the given Kirin's status is SCHEDULED :
    //  - UNKNOWN_EFFECT if delay is 0
    //  - SIGNIFICANT_DELAYS otherwise.
    delay: Option<i32>,
    // always present
    // is a unix_timestamp (nb of seconds since Jan 1st 1970)
    time: Option<i64>,

    // never present 
    uncertainty: Option<i32>,

    /// kirin extensions

    // always present, but deprecated use stop_time_event_status instead
    stop_time_event_relationship: Option<StopTimeUpdateScheduleRelationship>,

    // this will always be consistent with self.stop_time_event_relationship 
    stop_time_event_status: Option<StopTimeEventStatus>,
}

enum StopTimeEventStatus {
    // all these values can happen
    Scheduled,
    Deleted,
    Added,
    DeletedForDetour,
    AddedForDetour,
    // never happens
    NoData,
}
