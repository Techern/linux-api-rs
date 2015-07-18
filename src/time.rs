//!Constants and structures from include/uapi/linux/time.h

///A structure that contains the number of seconds and nanoseconds since an epoch.
///
///If in doubt, assume we're talking about the UNIX epoch.
pub struct timespec {
    ///The number of seconds contained in this timespec
    tv_sec: ::time_t,
    
    ///The number of nanoseconds contained in this timespec
    tv_nsec: ::c_long
}

///A structure that contains the number of seconds and microseconds since an epoch.
///
///If in doubt, assume we're talking about the UNIX epoch.
pub struct timeval {
    ///The number of seconds contained in this timeval
    tv_sec: ::time_t,
    
    ///The number of microseconds contained in this timeval
    tv_usec: ::suseconds_t
}

///A structure containing information on the time-based location of a timezone
///
///Please note that this does not include the name or country code, only the minutes west of Greenwich and the type of DST correction
pub struct timezone {
    ///The number of minutes west of Greenwich
    tz_minuteswest: ::c_int,
    
    ///The type of Daylight Savings Time correction
    tz_dsttime: ::c_int
}

//Names of the interval timers

///An interval timer that decrements in real time
///
///On expiration, a SIGALRM is delivered
pub const ITIMER_REAL: ::c_int = 0;

///An interval timer that decrements only when the process is executing.
///
///On expiration, a SIGVTALRM is delivered
pub const ITIMER_VIRTUAL: ::c_int = 1;

///Decrements both while the process is executing and while the system is executing on behalf of the process
///
///This is usually used to profile kernel-space and user-space concurrently. 
///
///If coupled with ITIMER_VIRTUAL, you can separate the two values - What is left when ITIMER_VIRTUAL's value is removed is kernel time
pub const ITIMER_PROF: ::c_int = 2;

///An interval timer based on a `timespec`
pub struct itimerspec {
    ///The period of time this timer should run for (Need to verify)
    it_interval: timespec,
    
    ///The amount of time left until expiration (Need to verify)
    it_value: timespec
}

///An interval timer based on a `timeval`
pub struct itimerval {
    ///The period of time this timer should run for (Need to verify)
    it_interval: timeval,
    
    ///The amount of time left until expiration (Need to verify)
    it_value: timeval
}

///A system-wide clock that measures time from the "real world"
///
///This clock **is** affected by discontinuous jumps in system time, NTP, and user changes
pub const CLOCK_REALTIME: ::clockid_t = 0;

///A clock that measures monotonic time since an unspecified starting point
///
///Unless you manage to break your system, this unspecified point is usually when your computer powers on.
///
///This is not affected by user changes, but is by `adjtime` and NTP.
pub const CLOCK_MONOTONIC: ::clockid_t = 1;

///A high-resolution per-process timer from the processor.
pub const CLOCK_PROCESS_CPUTIME_ID: ::clockid_t = 2;

///A (high-resolution?) thread-specific timer from the processor
pub const CLOCK_THREAD_CPUTIME_ID: ::clockid_t = 3;

///A hardware-based version of `CLOCK_MONOTONIC` that is not subject to changes
pub const CLOCK_MONOTONIC_RAW: ::clockid_t = 4;

///A faster but less precise version of `CLOCK_REALTIME`, measuring time in the "real world"
pub const CLOCK_REALTIME_COARSE: ::clockid_t = 5;

///A faster but less precise version of `CLOCK_MONOTONIC`, measuring time since an unspecified starting point
pub const CLOCK_MONOTONIC_COARSE: ::clockid_t = 6;

///Identical to `CLOCK_MONOTONIC`, but includes any time that the system is suspended.
pub const CLOCK_BOOTIME: ::clockid_t = 7;

///Identical to `CLOCK_REALTIME`, but timers exposed via this will wake the system if suspended
pub const CLOCK_REALTIME_ALARM: ::clockid_t = 8;

///Identical to `CLOCK_BOOTIME`, but timers exposed via this will wake the system if suspended
pub const CLOCK_BOOTTIME_ALARM: ::clockid_t = 9;

///A clock used for SGI systems. Need to investigate
pub const CLOCK_SGI_CYCLE: ::clockid_t = 10;

///A clock that shows International Atomic Time
pub const CLOCK_TAI: ::clockid_t = 11;

///The maximum clock ID that the system is allowed to have
pub const MAX_CLOCKS: ::clockid_t = 16; //Resolves to c_int. Please let me know if this should be c_int on it's own

///A mask for supported clocks
///
///Needs to be investigated
pub const CLOCKS_MASK: ::clockid_t = CLOCK_REALTIME | CLOCK_MONOTONIC;

///A shorthand variant of CLOCK_MONOTONIC.
///
///This isn't used in the kernel. Is it left over from an old change that was reverted?
pub const CLOCKS_MONO: ::clockid_t = CLOCK_MONOTONIC;

///A flag indicating time is absolute
pub const TIMER_ABSTIME: ::c_int = 0x01;