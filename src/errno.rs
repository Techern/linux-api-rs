//! Bindings to various errno.h definitions.

///Error numbers defined in errno-base.h, with some exceptions which are marked accordingly
pub mod errno_base {

    ///Operation not permitted
    pub const EPERM: ::c_int = 1;
    
    ///No such file or directory
    pub const ENOENT: ::c_int = 2;
    
    ///No such process
    pub const ESRCH: ::c_int = 3;
    
    ///Interrupted system call. Oops
    pub const EINTR: ::c_int = 4;
    
    ///I/O error.
    pub const EIO: ::c_int = 5;
    
    ///No such device or address
    pub const ENXIO: ::c_int = 6;
    
    ///Argument list too long.
    ///
    ///You thought it was error too big, didn't you?
    pub const E2BIG: ::c_int = 7;
    
    ///Exec format error
    pub const ENOEXEC: ::c_int = 8;
    
    ///Bad file number
    pub const EBADF: ::c_int = 9;
    
    ///No child processes
    ///
    ///Alternatively, your name is Anthony Dinozzo Jr or Richard Castle
    pub const ECHILD: ::c_int = 10;
    
    #[cfg(not(target_arch = "alpha"))]
    ///Try again, I'm not really an error, just an annoyance
    pub const EAGAIN: ::c_int = 11;
    
    #[cfg(target_arch = "alpha")]
    ///Try again, I'm not really an error, just an annoyance
    pub const EAGAIN: ::c_int = 35;
    
    ///Out of memory
    pub const ENOMEM: ::c_int = 12;
    
    ///Permission denied!
    pub const EACCES: ::c_int = 13;
    
    ///Bad address
    pub const EFAULT: ::c_int = 14;
    
    ///Block device required but not found
    pub const ENOTBLK: ::c_int = 15;
    
    ///Device or resource is too busy to listen to you right now
    pub const EBUSY: ::c_int = 16;
    
    ///File already exists
    pub const EEXIST: ::c_int = 17;
    
    ///Cross-device link. Sorry, but quantum entanglement is not working in storage drives yet
    pub const EXDEV: ::c_int = 18;
    
    ///No such device.
    ///
    ///Alternatively, you have been robbed by a Jedi knight. "This is not the device you were searching for"
    pub const ENODEV: ::c_int = 19;
    
    ///That's... not a directory
    pub const ENOTDIR: ::c_int = 20;
    
    ///That's... not a file. That's a directory
    pub const EISDIR: ::c_int = 21;
    
    ///Your argument is invalid!
    pub const EINVAL: ::c_int = 22;
    
    ///File table overflowed
    pub const ENFILE: ::c_int = 23;
    
    ///Too many files are already open
    pub const EMFILE: ::c_int = 24;
    
    ///You are not a typewriter
    ///
    ///Does this expand to you are not a terminal? What a about a smart kettle? Or a WIFI-enabled microwave? Hey, somebody should go patent that
    pub const ENOTTTY: ::c_int = 25;
    
    ///Text file is too busy to listen to you right now
    pub const ETXTBSY: ::c_int = 26; //Ooooh, what are you writing?
    
    ///File is too large. What are you doing, downloading a car?
    pub const EFBIG: ::c_int = 27; //Okay, really, how?
    
    ///You don't have any (or at least enough) space left on the specified device. You might need a new hard drive.
    ///
    ///Might I suggest a SSD? They cost more but the performance gain is worth it
    pub const ENOSPC: ::c_int = 28; //I'm so sorry
    
    ///Illegal seek
    ///
    ///What the hell are you doing?
    pub const ESPIPE: ::c_int = 29;
    
    ///You're trying to write to a read-only file system.
    pub const EROFS: ::c_int = 30; //Haha, nice try
    
    ///Too many links exist
    pub const EMLINK: ::c_int = 31;
    
    ///Broken pipe
    pub const EPIPE: ::c_int = 32;
    
    ///Math argument is out of the function's domain
    ///
    ///??? I need more coffee to get this. Is it like trying to check if an int is NaN?
    pub const EDOM: ::c_int = 33;
    
    ///Math result could not be represented
    ///
    ///Stop trying to divide by ze--- ```EACCES```
    pub const ERANGE: ::c_int = 34;

}

pub use self::errno_base::*;

#[cfg(not(any(target_arch = "alpha", target_arch = "mips", target_arch = "parisc", target_arch = "sparc")))]
pub mod errno_common {

    ///Resource deadlock would occur
    pub const EDEADLK: ::c_int = 35;
    
    ///File name is too long. (Hah!)
    pub const ENAMETOOLONG: ::c_int = 36;
    
    ///No record locks available
    pub const ENOLCK: ::c_int = 37;
    
    ///Invalid system call number
    ///
    ///This is special. Arch syscall entry code returns -ENOSYS if 
    ///users try to call a syscall that doesn't exist.
    pub const ENOSYS: ::c_int = 38;
    
    ///Directory is not empty.
    pub const ENOTEMPTY: ::c_int = 39;
    
    ///Too many symbolic links encountered. This is probably an infinite loop
    ///
    ///Alternatively, you broke your computer.
    pub const ELOOP: ::c_int = 40;
    
    ///Operation would block. You should try again?
    pub const EWOULDBLOCK: ::c_int = ::EAGAIN;
    
    ///No message of the desired type
    pub const ENOMSG: ::c_int = 42;
    
    ///Identifier was removed
    pub const EIDRM: ::c_int = 43;
    
    ///Channel number is out of acceptable range
    pub const ECHRNG: ::c_int = 44;
    
    ///Level 2 is not synchronized
    pub const EL2NSYNC: ::c_int = 45;
    
    ///Level 3 was halted
    pub const EL3HLT: ::c_int = 46;
    
    ///Level 3 was reset
    pub const EL3RST: ::c_int = 47;

    ///Link number is out of range
    ///
    ///How do you do this?
    pub const ELNRNG: ::c_int = 48;
    
    ///Protocol driver not attached
    pub const EUNATCH: ::c_int = 49;
    
    ///No CSI structure is available
    pub const ENOCSI: ::c_int = 50;
    
    ///Level 2 was halted
    pub const EL2HLT: ::c_int = 51;
    
    ///Invalid exchange
    pub const EBADE: ::c_int = 52;
    
    ///Invalid request descriptor
    pub const EBADR: ::c_int = 53;
    
    ///Exchange is full.
    ///
    ///I am so sorry
    pub const EXFULL: ::c_int = 54;
    
    ///No anode is available
    pub const ENOANO: ::c_int = 55;
    
    ///Invalid request code
    pub const EBADRQC: ::c_int = 56;
    
    ///Invalid slot
    pub const EBADSLT: ::c_int = 57;
    
    #[cfg(not(target_arch = "powerpc"))]
    ///A deadlock would occur, again
    pub const EDEADLOCK: ::c_int = EDEADLK;
    
    #[cfg(target_arch = "powerpc")]
    ///A deadlock would occur, again
    pub const EDEADLOCK: ::c_int = 58;
    
    ///Bad font file format
    pub const EBFONT: ::c_int = 59;
    
    ///Device is not a stream
    pub const ENOSTR: ::c_int = 60;
    
    ///No data is available
    pub const ENODATA: ::c_int = 61;
    
    ///Timer expired
    pub const ETIME: ::c_int = 62;
    
    ///Out of streams resources
    ///
    ///Should this be stream's? I'm not sure about the context. Is it the resources of the stream, or resources of the system's supply of streams?
    pub const ENOSR: ::c_int = 63;
    
    ///Requested machine was not on the network
    pub const ENONET: ::c_int = 64;
    
    ///Package is not installed
    pub const ENOPKG: ::c_int = 65;
    
    ///Object is remote when it is required to be local
    pub const EREMOTE: ::c_int = 66;
    
    ///Link has been severed
    pub const ENOLINK: ::c_int = 67;
    
    ///Error in advertising
    pub const EADV: ::c_int = 68;
    
    ///Srmount error
    pub const ESRMNT: ::c_int = 69;
    
    ///Communications error on send
    pub const ECOMM: ::c_int = 70;
    
    ///Protocol error
    pub const EPROTO: ::c_int = 71;
    
    ///A multihop was attempted
    pub const EMULTIHOP: ::c_int = 72;
    
    ///A RFS-specific error occurred
    pub const EDOTDOT: ::c_int = 73;
    
    ///Message is not a data message
    pub const EBADMSG: ::c_int = 74;
    
    ///Value is too large for the defined data type
    pub const EOVERFLOW: ::c_int = 75;
    
    ///Name is not unique on the network
    ///
    ///No, BILLY_TABLES, you can't have ten computers called '; DROP TABLE users' for "backup purposes"
    pub const ENOTUNIQ: ::c_int = 76;
    
    ///File descriptor is in a bad state
    ///
    ///Huh. I haven't seen this before
    pub const EBADFD: ::c_int = 77;
    
    ///The remote address changed.
    ///
    ///How does this happen? A change from wifi to 4g, etc?
    pub const EREMCHG: ::c_int = 78;
    
    ///Can not access a needed shared library
    ///
    ///Thankfully, with Cargo, this shouldn't happen :)
    pub const ELIBACC: ::c_int = 79;
    
    ///Attempted to access a shared library, only to find out it was corrupted
    ///
    ///WELL, FUCK.
    pub const ELIBBAD: ::c_int = 80;
    
    ///.lib section in a.out is corrupted
    pub const ELIBSCN: ::c_int = 81;
    
    ///Attempting to link in too many shared libraries
    ///
    ///Are... are you okay?
    pub const ELIBMAX: ::c_int = 82;
    
    ///Cannot exec a shared library directly
    pub const ELIBEXEC: ::c_int = 83;
    
    ///Illegal byte sequence
    ///
    ///I don't know what the hell you are doing, but you're probably doing it wrong
    pub const EILSEQ: ::c_int = 84;
    
    ///Interrupted system call, you should restart it
    ///
    ///Well, that's easy
    pub const ERESTART: ::c_int = 85;
    
    ///Streams pipe error
    pub const ESTRPIPE: ::c_int = 86;
    
    ///Too many users
    ///
    ///*Points finger and laughs*
    pub const EUSERS: ::c_int = 87;
    
    ///Socket operation attempted on an object that is not a socket
    pub const ENOTSOCK: ::c_int = 88;
    
    ///Destination address required
    pub const EDESTADDRREQ: ::c_int = 89;
    
    ///Message is too long
    pub const EMSGSIZE: ::c_int = 90;
    
    ///Protocol type is wrong for this socket
    pub const EPROTOTYPE: ::c_int = 91;
    
    ///Protocol is not available
    pub const ENOPROTOOPT: ::c_int = 92;
    
    ///Protocol is not supported
    pub const EPROTONOSUPPORT: ::c_int = 93;
    
    ///Socket type is not supported
    ///
    ///What?
    pub const ESOCKTNOSUPPORT: ::c_int = 94;
    
    ///Operation not supported on transport endpoint
    pub const EOPNOTSUPP: ::c_int = 95;
    
    ///Protocol family is not supported
    pub const EPFNOSUPPORT: ::c_int = 96;
    
    ///Address family is not supported by this protocol
    pub const EAFNOSUPPORT: ::c_int = 97;
    
    ///Address is already in use
    pub const EADDRINUSE: ::c_int = 98;
    
    ///Cannot assign the requested address
    pub const EADDRNOTAVAIL: ::c_int = 99;
    
    ///Network is down
    pub const ENETDOWN: ::c_int = 100;
    
    ///Network is unreachable :(
    pub const ENETUNREACH: ::c_int = 101;
    
    ///Network dropped connection because of a reset
    pub const ENETRESET: ::c_int = 102;
    
    ///Software caused the connection to abort
    pub const ECONNABORTED: ::c_int = 103;
    
    ///Connection was reset by a peer
    pub const ECONNRESET: ::c_int = 104;
    
    ///No buffer space is available
    pub const ENOBUFS: ::c_int = 105;
    
    ///Transport endpoint is already connected
    pub const EISCONN: ::c_int = 106;
    
    ///Transport endpoint is not connected
    pub const ENOTCONN: ::c_int = 107;
    
    ///Cannot send after the transport endpoint is shut down
    pub const ESHUTDOWN: ::c_int = 108;
    
    ///Too many references; Cannot splice
    pub const ETOOMANYREFS: ::c_int = 109;
    
    ///Connection timed out
    pub const ETIMEDOUT: ::c_int = 110;
    
    ///Connection was refused
    pub const ECONNREFUSED: ::c_int = 111;
    
    ///The host is down! D:
    pub const EHOSTDOWN: ::c_int = 112;
    
    ///There is no usable route to the host
    pub const EHOSTUNREACH: ::c_int = 113;
    
    ///Operation is already in progress. Be patient!
    pub const EALREADY: ::c_int = 114;
    
    ///Operation is in progress
    pub const EINPROGRESS: ::c_int = 115;
    
    ///The file handle is stale. Ew!
    pub const ESTALE: ::c_int = 116;
    
    ///The structure needs cleaning
    pub const EUCLEAN: ::c_int = 117;
    
    ///This is not a XENIX named type file
    pub const ENOTNAM: ::c_int = 118;
    
    ///No XENIX semaphores are available
    pub const ENAVAIL: ::c_int = 119;
    
    ///This is a named type file
    pub const EISNAM: ::c_int = 120;
    
    ///A remote I/O error occurred
    pub const EREMOTEIO: ::c_int = 121;
    
    ///You exceeded your quota
    pub const EDQUOT: ::c_int = 122;
    
    ///No medium was found
    pub const ENOMEDIUM: ::c_int = 123;
    
    ///You're using the wrong medium type
    pub const EMEDIUMTYPE: ::c_int = 124;
    
    ///The operation was cancelled
    pub const ECANCELED: ::c_int = 125;
    
    ///The required key is not available
    pub const ENOKEY: ::c_int = 126;
    
    ///The key you're using has expired
    pub const EKEYEXPIRED: ::c_int = 127;
    
    ///The key you're using has been revoked
    pub const EKEYREVOKED: ::c_int = 128;
    
    ///The key you're using was rejected by the service you're trying to access
    pub const EKEYREJECTED: ::c_int = 129;
    
    ///Your owner died.
    ///
    ///Dobby is a free computer!
    ///
    ///Just kidding. Your mutex's owner died. Dobby is still a slave
    pub const EOWNERDEAD: ::c_int = 130;
    
    ///The state of your mutex is not recoverable
    pub const ENOTRECOVERABLE: ::c_int = 131;
    
    ///Operation is not possible due to RF-kill
    pub const ERFKILL: ::c_int = 132;
    
    ///The memory page has a hardware error
    pub const EHWPOISON: ::c_int = 133;

}

#[cfg(not(any(target_arch = "alpha", target_arch = "mips", target_arch = "parisc", target_arch = "sparc")))]
pub use self::errno_common::*;

#[cfg(target_arch = "alpha")]
pub mod errno_alpha {

    ///Resource deadlock would occur
    pub const EDEADLK: ::c_int = 11;
    
    ///Operation would block. You should try again?
    pub const EWOULDBLOCK: ::c_int = ::EAGAIN;
    
    ///Operation is in progress
    pub const EINPROGRESS: ::c_int = 36;
    
    ///Operation is already in progress. Be patient!
    pub const EALREADY: ::c_int = 37;
    
    ///Socket operation attempted on an object that is not a socket
    pub const ENOTSOCK: ::c_int = 38;
    
    ///Destination address required
    pub const EDESTADDRREQ: ::c_int = 39;
    
    ///Message is too long
    pub const EMSGSIZE: ::c_int = 40;
    
    ///Protocol type is wrong for this socket
    pub const EPROTOTYPE: ::c_int = 41;
    
    ///Protocol is not available
    pub const ENOPROTOOPT: ::c_int = 42;
    
    ///Protocol is not supported
    pub const EPROTONOSUPPORT: ::c_int = 43;
    
    ///Socket type is not supported
    ///
    ///What?
    pub const ESOCKTNOSUPPORT: ::c_int = 44;
    
    ///Operation not supported on transport endpoint
    pub const EOPNOTSUPP: ::c_int = 45;
    
    ///Protocol family is not supported
    pub const EPFNOSUPPORT: ::c_int = 46;
    
    ///Address family is not supported by this protocol
    pub const EAFNOSUPPORT: ::c_int = 47;
    
    ///Address is already in use
    pub const EADDRINUSE: ::c_int = 48;
    
    ///Cannot assign the requested address
    pub const EADDRNOTAVAIL: ::c_int = 49;
    
    ///Network is down
    pub const ENETDOWN: ::c_int = 50;
    
    ///Network is unreachable :(
    pub const ENETUNREACH: ::c_int = 51;
    
    ///Network dropped connection because of a reset
    pub const ENETRESET: ::c_int = 52;
    
    ///Software caused the connection to abort
    pub const ECONNABORTED: ::c_int = 53;
    
    ///Connection was reset by a peer
    pub const ECONNRESET: ::c_int = 54;
    
    ///No buffer space is available
    pub const ENOBUFS: ::c_int = 55;
    
    ///Transport endpoint is already connected
    pub const EISCONN: ::c_int = 56;
    
    ///Transport endpoint is not connected
    pub const ENOTCONN: ::c_int = 57;
    
    ///Cannot send after the transport endpoint is shut down
    pub const ESHUTDOWN: ::c_int = 58;
    
    ///Too many references; Cannot splice
    pub const ETOOMANYREFS: ::c_int = 59;
    
    ///Connection timed out
    pub const ETIMEDOUT: ::c_int = 60;
    
    ///Connection was refused
    pub const ECONNREFUSED: ::c_int = 61;
    
    ///Too many symbolic links encountered. This is probably an infinite loop
    ///
    ///Alternatively, you broke your computer.
    pub const ELOOP: ::c_int = 62;
    
    ///File name is too long. (Hah!)
    pub const ENAMETOOLONG: ::c_int = 63;
    
    ///The host is down! D:
    pub const EHOSTDOWN: ::c_int = 64;
    
    ///There is no usable route to the host
    pub const EHOSTUNREACH: ::c_int = 65;
    
    ///Directory is not empty.
    pub const ENOTEMPTY: ::c_int = 66;
    
    //67 unused?
    
    ///Too many users
    ///
    ///*Points finger and laughs*
    pub const EUSERS: ::c_int = 68;
    
    ///You exceeded your quota
    pub const EDQUOT: ::c_int = 69;
    
    ///The file handle is stale. Ew!
    pub const ESTALE: ::c_int = 70;
    
    ///Object is remote when it is required to be local
    pub const EREMOTE: ::c_int = 71;
    
    //71 - 76 unused?
    
    ///No record locks available
    pub const ENOLCK: ::c_int = 77;
    
    ///Invalid system call number
    ///
    ///This is special. Arch syscall entry code returns -ENOSYS if 
    ///users try to call a syscall that doesn't exist.
    pub const ENOSYS: ::c_int = 78;
    
    //79 unused
    
    ///No message of the desired type
    pub const ENOMSG: ::c_int = 80;
    
    ///Identifier was removed
    pub const EIDRM: ::c_int = 81;
    
    ///Out of streams resources
    ///
    ///Should this be stream's? I'm not sure about the context. Is it the resources of the stream, or resources of the system's supply of streams?
    pub const ENOSR: ::c_int = 82;
    
    ///Timer expired
    pub const ETIME: ::c_int = 83;
    
    ///Message is not a data message
    pub const EBADMSG: ::c_int = 84;
    
    ///Protocol error
    pub const EPROTO: ::c_int = 85;
    
    ///No data is available
    pub const ENODATA: ::c_int = 86;
    
    ///Device is not a stream
    pub const ENOSTR: ::c_int = 87;
    
    
        
    ///Package is not installed
    pub const ENOPKG: ::c_int = 92;
    
    
    
    ///Illegal byte sequence
    ///
    ///I don't know what the hell you are doing, but you're probably doing it wrong
    pub const EILSEQ: ::c_int = 116;
    
    //"Random noise" my arse
    
    ///Channel number is out of acceptable range
    pub const ECHRNG: ::c_int = 88;
    
    ///Level 2 is not synchronized
    pub const EL2NSYNC: ::c_int = 89;
    
    ///Level 3 was halted
    pub const EL3HLT: ::c_int = 90;
    
    ///Level 3 was reset
    pub const EL3RST: ::c_int = 91;
    
    
    
    ///Link number is out of range
    ///
    ///How do you do this?
    pub const ELNRNG: ::c_int = 93;
    
    ///Protocol driver not attached
    pub const EUNATCH: ::c_int = 94;
    
    ///No CSI structure is available
    pub const ENOCSI: ::c_int = 95;
    
    ///Level 2 was halted
    pub const EL2HLT: ::c_int = 96;
    
    ///Invalid exchange
    pub const EBADE: ::c_int = 97;
    
    ///Invalid request descriptor
    pub const EBADR: ::c_int = 98;
    
    ///Exchange is full.
    ///
    ///I am so sorry
    pub const EXFULL: ::c_int = 99;
    
    ///No anode is available
    pub const ENOANO: ::c_int = 100;
    
    ///Invalid request code
    pub const EBADRQC: ::c_int = 101;
    
    ///Invalid slot
    pub const EBADSLT: ::c_int = 102;
    
    ///A deadlock would occur, again
    pub const EDEADLOCK: ::c_int = EDEADLK;
    
    
    
    ///Bad font file format
    pub const EBFONT: ::c_int = 104;
    
    ///Requested machine was not on the network
    pub const ENONET: ::c_int = 105;
    
    ///Link has been severed
    pub const ENOLINK: ::c_int = 106;
    
    ///Error in advertising
    pub const EADV: ::c_int = 107;
    
    ///Srmount error
    pub const ESRMNT: ::c_int = 108;
    
    ///Communications error on send
    pub const ECOMM: ::c_int = 109;
    
    ///A multihop was attempted
    pub const EMULTIHOP: ::c_int = 110;
    
    ///A RFS-specific error occurred
    pub const EDOTDOT: ::c_int = 111;
    
    ///Value is too large for the defined data type
    pub const EOVERFLOW: ::c_int = 112;
    
    ///Name is not unique on the network
    ///
    ///No, BILLY_TABLES, you can't have ten computers called '; DROP TABLE users' for "backup purposes"
    pub const ENOTUNIQ: ::c_int = 113;
    
    ///File descriptor is in a bad state
    ///
    ///Huh. I haven't seen this before
    pub const EBADFD: ::c_int = 114;
    
    ///The remote address changed.
    ///
    ///How does this happen? A change from wifi to 4g, etc?
    pub const EREMCHG: ::c_int = 115;
    
    
    
    ///The structure needs cleaning
    pub const EUCLEAN: ::c_int = 117;
    
    ///This is not a XENIX named type file
    pub const ENOTNAM: ::c_int = 118;
    
    ///No XENIX semaphores are available
    pub const ENAVAIL: ::c_int = 119;
    
    ///This is a named type file
    pub const EISNAM: ::c_int = 120;
    
    ///A remote I/O error occurred
    pub const EREMOTEIO: ::c_int = 121;
    
    
    
    ///Can not access a needed shared library
    ///
    ///Thankfully, with Cargo, this shouldn't happen :)
    pub const ELIBACC: ::c_int = 122;
    
    ///Attempted to access a shared library, only to find out it was corrupted
    ///
    ///WELL, FUCK.
    pub const ELIBBAD: ::c_int = 123;
    
    ///.lib section in a.out is corrupted
    pub const ELIBSCN: ::c_int = 124;
    
    ///Attempting to link in too many shared libraries
    ///
    ///Are... are you okay?
    pub const ELIBMAX: ::c_int = 125;
    
    ///Cannot exec a shared library directly
    pub const ELIBEXEC: ::c_int = 126;
    
    ///Interrupted system call, you should restart it
    ///
    ///Well, that's easy
    pub const ERESTART: ::c_int = 127;
    
    ///Streams pipe error
    pub const ESTRPIPE: ::c_int = 128;
    
    
    
    ///No medium was found
    pub const ENOMEDIUM: ::c_int = 129;
    
    ///You're using the wrong medium type
    pub const EMEDIUMTYPE: ::c_int = 130;
    
    ///The operation was cancelled
    pub const ECANCELED: ::c_int = 131;
    
    ///The required key is not available
    pub const ENOKEY: ::c_int = 132;
    
    ///The key you're using has expired
    pub const EKEYEXPIRED: ::c_int = 133;
    
    ///The key you're using has been revoked
    pub const EKEYREVOKED: ::c_int = 134;
    
    ///The key you're using was rejected by the service you're trying to access
    pub const EKEYREJECTED: ::c_int = 135;
    
    
    
    ///Your owner died.
    ///
    ///Dobby is a free computer!
    ///
    ///Just kidding. Your mutex's owner died. Dobby is still a slave
    pub const EOWNERDEAD: ::c_int = 136;
    
    ///The state of your mutex is not recoverable
    pub const ENOTRECOVERABLE: ::c_int = 137;
    
    
    
    ///Operation is not possible due to RF-kill
    pub const ERFKILL: ::c_int = 138;
    
    
    
    ///The memory page has a hardware error
    pub const EHWPOISON: ::c_int = 139;

}

#[cfg(target_arch = "alpha")]
pub use self::errno_alpha::*;

#[cfg(target_arch = "mips")]
pub mod errno_mips {
    
    ///No message of the desired type
    pub const ENOMSG: ::c_int = 35;
    
    ///Identifier was removed
    pub const EIDRM: ::c_int = 36;
    
    ///Channel number is out of acceptable range
    pub const ECHRNG: ::c_int = 37;
    
    ///Level 2 is not synchronized
    pub const EL2NSYNC: ::c_int = 38;
    
    ///Level 3 was halted
    pub const EL3HLT: ::c_int = 39;
    
    ///Level 3 was reset
    pub const EL3RST: ::c_int = 40;

    ///Link number is out of range
    ///
    ///How do you do this?
    pub const ELNRNG: ::c_int = 41;
    
    ///Protocol driver not attached
    pub const EUNATCH: ::c_int = 42;
    
    ///No CSI structure is available
    pub const ENOCSI: ::c_int = 43;
    
    ///Level 2 was halted
    pub const EL2HLT: ::c_int = 44;
    
    ///Resource deadlock would occur
    pub const EDEADLK: ::c_int = 45;
    
    ///No record locks available
    pub const ENOLCK: ::c_int = 46;
    
    ///Invalid exchange
    pub const EBADE: ::c_int = 50;
    
    ///Invalid request descriptor
    pub const EBADR: ::c_int = 51;
    
    ///Exchange is full.
    pub const EXFULL: ::c_int = 52;
    
    ///No anode is available
    pub const ENOANO: ::c_int = 53;
    
    ///Invalid request code
    pub const EBADRQC: ::c_int = 54;
    
    ///Invalid slot
    pub const EBADSLT: ::c_int = 55;
    
    ///A deadlock would occur, again
    pub const EDEADLOCK: ::c_int = 56;
    
    ///Bad font file format
    pub const EBFONT: ::c_int = 59;
    
    ///Device is not a stream
    pub const ENOSTR: ::c_int = 60;
    
    ///No data is available
    pub const ENODATA: ::c_int = 61;
    
    ///Timer expired
    pub const ETIME: ::c_int = 62;
    
    ///Out of streams resources
    ///
    ///Should this be stream's? I'm not sure about the context. Is it the resources of the stream, or resources of the system's supply of streams?
    pub const ENOSR: ::c_int = 63;
    
    ///Requested machine was not on the network
    pub const ENONET: ::c_int = 64;
    
    ///Package is not installed
    pub const ENOPKG: ::c_int = 65;
    
    ///Object is remote when it is required to be local
    pub const EREMOTE: ::c_int = 66;
    
    ///Link has been severed
    pub const ENOLINK: ::c_int = 67;
    
    ///Error in advertising
    pub const EADV: ::c_int = 68;
    
    ///Srmount error
    pub const ESRMNT: ::c_int = 69;
    
    ///Communications error on send
    pub const ECOMM: ::c_int = 70;
    
    ///Protocol error
    pub const EPROTO: ::c_int = 71;
    
    ///A RFS-specific error occurred
    pub const EDOTDOT: ::c_int = 73;
    
    ///A multihop was attempted
    pub const EMULTIHOP: ::c_int = 74;
    
    ///Message is not a data message
    pub const EBADMSG: ::c_int = 77;
    
    ///File name is too long. (Hah!)
    pub const ENAMETOOLONG: ::c_int = 78;
    
    ///Value is too large for the defined data type
    pub const EOVERFLOW: ::c_int = 79;
    
    ///Name is not unique on the network
    pub const ENOTUNIQ: ::c_int = 80;
    
    ///File descriptor is in a bad state
    ///
    ///Huh. I haven't seen this before
    pub const EBADFD: ::c_int = 81;
    
    ///The remote address changed.
    ///
    ///How does this happen? A change from wifi to 4g, etc?
    pub const EREMCHG: ::c_int = 82;
    
    ///Can not access a needed shared library
    ///
    ///Thankfully, with Cargo, this shouldn't happen :)
    pub const ELIBACC: ::c_int = 83;
    
    ///Attempted to access a shared library, only to find out it was corrupted
    pub const ELIBBAD: ::c_int = 84;
    
    ///.lib section in a.out is corrupted
    pub const ELIBSCN: ::c_int = 85;
    
    ///Attempting to link in too many shared libraries
    pub const ELIBMAX: ::c_int = 86;
    
    ///Cannot exec a shared library directly
    pub const ELIBEXEC: ::c_int = 87;
    
    ///Illegal byte sequence
    pub const EILSEQ: ::c_int = 88;
    
    ///Invalid system call number
    ///
    ///This is special. Arch syscall entry code returns -ENOSYS if 
    ///users try to call a syscall that doesn't exist.
    pub const ENOSYS: ::c_int = 89;
    
    ///Too many symbolic links encountered. This is probably an infinite loop
    pub const ELOOP: ::c_int = 90;
    
    ///Interrupted system call, you should restart it
    pub const ERESTART: ::c_int = 91;
    
    ///Streams pipe error
    pub const ESTRPIPE: ::c_int = 92;
    
    ///Directory is not empty.
    pub const ENOTEMPTY: ::c_int = 93;
    
    ///Too many users
    pub const EUSERS: ::c_int = 94;
    
    ///Socket operation attempted on an object that is not a socket
    pub const ENOTSOCK: ::c_int = 95;
    
    ///Destination address required
    pub const EDESTADDRREQ: ::c_int = 96;
    
    ///Message is too long
    pub const EMSGSIZE: ::c_int = 97;
    
    ///Protocol type is wrong for this socket
    pub const EPROTOTYPE: ::c_int = 98;
    
    ///Protocol is not available
    pub const ENOPROTOOPT: ::c_int = 99;
    
    ///Protocol is not supported
    pub const EPROTONOSUPPORT: ::c_int = 120;
    
    ///Socket type is not supported
    ///
    ///What?
    pub const ESOCKTNOSUPPORT: ::c_int = 121;
    
    ///Operation not supported on transport endpoint
    pub const EOPNOTSUPP: ::c_int = 122;
    
    ///Protocol family is not supported
    pub const EPFNOSUPPORT: ::c_int = 123;
    
    ///Address family is not supported by this protocol
    pub const EAFNOSUPPORT: ::c_int = 124;
    
    ///Address is already in use
    pub const EADDRINUSE: ::c_int = 125;
    
    ///Cannot assign the requested address
    pub const EADDRNOTAVAIL: ::c_int = 126;
    
    ///Network is down
    pub const ENETDOWN: ::c_int = 127;
    
    ///Network is unreachable :(
    pub const ENETUNREACH: ::c_int = 128;
    
    ///Network dropped connection because of a reset
    pub const ENETRESET: ::c_int = 129;
    
    ///Software caused the connection to abort
    pub const ECONNABORTED: ::c_int = 130;
    
    ///Connection was reset by a peer
    pub const ECONNRESET: ::c_int = 131;
    
    ///No buffer space is available
    pub const ENOBUFS: ::c_int = 132;
    
    ///Transport endpoint is already connected
    pub const EISCONN: ::c_int = 133;
    
    ///Transport endpoint is not connected
    pub const ENOTCONN: ::c_int = 134;
    
    ///The structure needs cleaning
    pub const EUCLEAN: ::c_int = 135;
    
    ///This is not a XENIX named type file
    pub const ENOTNAM: ::c_int = 137;
    
    ///No XENIX semaphores are available
    pub const ENAVAIL: ::c_int = 138;
    
    ///This is a named type file
    pub const EISNAM: ::c_int = 139;
    
    ///A remote I/O error occurred
    pub const EREMOTEIO: ::c_int = 140;
    
    ///Reserved for future use
    pub const EINIT: ::c_int = 141;
    
    ///Error 142, seems to be MIPS specific
    pub const EREMDEV: ::c_int = 142;
    
    ///Cannot send after the transport endpoint is shut down
    pub const ESHUTDOWN: ::c_int = 143;
    
    ///Too many references; Cannot splice
    pub const ETOOMANYREFS: ::c_int = 144;
    
    ///Connection timed out
    pub const ETIMEDOUT: ::c_int = 145;
    
    ///Connection was refused
    pub const ECONNREFUSED: ::c_int = 146;
    
    ///The host is down! D:
    pub const EHOSTDOWN: ::c_int = 147;
    
    ///There is no usable route to the host
    pub const EHOSTUNREACH: ::c_int = 148;
    
    ///Operation would block. You should try again?
    pub const EWOULDBLOCK: ::c_int = ::EAGAIN;
    
    ///Operation is already in progress. Be patient!
    pub const EALREADY: ::c_int = 149;
    
    ///Operation is in progress
    pub const EINPROGRESS: ::c_int = 150;
    
    ///The file handle is stale.
    pub const ESTALE: ::c_int = 151;
    
    ///The operation was cancelled
    pub const ECANCELED: ::c_int = 158;
    
    ///No medium was found
    pub const ENOMEDIUM: ::c_int = 159;
    
    ///You're using the wrong medium type
    pub const EMEDIUMTYPE: ::c_int = 160;
    
    ///The required key is not available
    pub const ENOKEY: ::c_int = 161;
    
    ///The key you're using has expired
    pub const EKEYEXPIRED: ::c_int = 162;
    
    ///The key you're using has been revoked
    pub const EKEYREVOKED: ::c_int = 163;
    
    ///The key you're using was rejected by the service you're trying to access
    pub const EKEYREJECTED: ::c_int = 164;
    
    ///Your mutex's owner died. 
    pub const EOWNERDEAD: ::c_int = 165;
    
    ///The state of your mutex is not recoverable
    pub const ENOTRECOVERABLE: ::c_int = 166;
    
    
    
    ///Operation is not possible due to RF-kill
    pub const ERFKILL: ::c_int = 167;
    
    
    
    ///The memory page has a hardware error
    pub const EHWPOISON: ::c_int = 168;
    
    
    ///You exceeded your quota
    pub const EDQUOT: ::c_int = 1133;

}

#[cfg(target_arch = "mips")]
pub use self::errno_mips::*;

#[cfg(target_arch = "parisc")]
pub mod errno_parisc {
    
    ///No message of the desired type
    pub const ENOMSG: ::c_int = 35;
    
    ///Identifier was removed
    pub const EIDRM: ::c_int = 36;
    
    ///Channel number is out of acceptable range
    pub const ECHRNG: ::c_int = 37;
    
    ///Level 2 is not synchronized
    pub const EL2NSYNC: ::c_int = 38;
    
    ///Level 3 was halted
    pub const EL3HLT: ::c_int = 39;
    
    ///Level 3 was reset
    pub const EL3RST: ::c_int = 40;

    ///Link number is out of range
    ///
    ///How do you do this?
    pub const ELNRNG: ::c_int = 41;
    
    ///Protocol driver not attached
    pub const EUNATCH: ::c_int = 42;
    
    ///No CSI structure is available
    pub const ENOCSI: ::c_int = 43;
    
    ///Level 2 was halted
    pub const EL2HLT: ::c_int = 44;
    
    ///Resource deadlock would occur
    pub const EDEADLK: ::c_int = 45;
    
    ///A deadlock would occur, again
    pub const EDEADLOCK: ::c_int = EDEADLK;
    
    ///No record locks available
    pub const ENOLCK: ::c_int = 46;
    
    ///Illegal byte sequence
    pub const EILSEQ: ::c_int = 47;
    
    ///Requested machine was not on the network
    pub const ENONET: ::c_int = 50;
    
    ///No data is available
    pub const ENODATA: ::c_int = 51;
    
    ///Timer expired
    pub const ETIME: ::c_int = 52;
    
    ///Out of streams resources
    ///
    ///Should this be stream's? I'm not sure about the context. Is it the resources of the stream, or resources of the system's supply of streams?
    pub const ENOSR: ::c_int = 53;
    
    ///Device is not a stream
    pub const ENOSTR: ::c_int = 54;
    
    ///Package is not installed
    pub const ENOPKG: ::c_int = 55;
    
    ///Link has been severed
    pub const ENOLINK: ::c_int = 57;
    
    ///Error in advertising
    pub const EADV: ::c_int = 58;
    
    ///Srmount error
    pub const ESRMNT: ::c_int = 59;
    
    ///Communications error on send
    pub const ECOMM: ::c_int = 60;
    
    ///Protocol error
    pub const EPROTO: ::c_int = 61;
    
    
    
    ///A multihop was attempted
    pub const EMULTIHOP: ::c_int = 64;
    
    
    
    ///A RFS-specific error occurred
    pub const EDOTDOT: ::c_int = 66;
    
    ///Message is not a data message
    pub const EBADMSG: ::c_int = 67;
    
    ///Too many users
    pub const EUSERS: ::c_int = 68;
    
    ///You exceeded your quota
    pub const EDQUOT: ::c_int = 69;
    
    ///The file handle is stale.
    pub const ESTALE: ::c_int = 70;
    
    ///Object is remote when it is required to be local
    pub const EREMOTE: ::c_int = 71;
    
    ///Value is too large for the defined data type
    pub const EOVERFLOW: ::c_int = 72;
    
    
    
    ///Invalid exchange
    pub const EBADE: ::c_int = 160;
    
    ///Invalid request descriptor
    pub const EBADR: ::c_int = 161;
    
    ///Exchange is full.
    pub const EXFULL: ::c_int = 162;
    
    ///No anode is available
    pub const ENOANO: ::c_int = 163;
    
    ///Invalid request code
    pub const EBADRQC: ::c_int = 164;
    
    ///Invalid slot
    pub const EBADSLT: ::c_int = 165;
    
    ///Bad font file format
    pub const EBFONT: ::c_int = 166;
    
    ///Name is not unique on the network
    pub const ENOTUNIQ: ::c_int = 167;
    
    ///File descriptor is in a bad state
    ///
    ///Huh. I haven't seen this before
    pub const EBADFD: ::c_int = 168;
    
    ///The remote address changed.
    ///
    ///How does this happen? A change from wifi to 4g, etc?
    pub const EREMCHG: ::c_int = 169;
    
    ///Can not access a needed shared library
    ///
    ///Thankfully, with Cargo, this shouldn't happen :)
    pub const ELIBACC: ::c_int = 170;
    
    ///Attempted to access a shared library, only to find out it was corrupted
    pub const ELIBBAD: ::c_int = 171;
    
    ///.lib section in a.out is corrupted
    pub const ELIBSCN: ::c_int = 172;
    
    ///Attempting to link in too many shared libraries
    pub const ELIBMAX: ::c_int = 173;
    
    ///Cannot exec a shared library directly
    pub const ELIBEXEC: ::c_int = 174;
    
    ///Interrupted system call, you should restart it
    pub const ERESTART: ::c_int = 175;
    
    ///Streams pipe error
    pub const ESTRPIPE: ::c_int = 176;
    
    ///The structure needs cleaning
    pub const EUCLEAN: ::c_int = 177;
    
    ///This is not a XENIX named type file
    pub const ENOTNAM: ::c_int = 178;
    
    ///No XENIX semaphores are available
    pub const ENAVAIL: ::c_int = 179;
    
    ///This is a named type file
    pub const EISNAM: ::c_int = 180;
    
    ///A remote I/O error occurred
    pub const EREMOTEIO: ::c_int = 181;
    
    ///No medium was found
    pub const ENOMEDIUM: ::c_int = 182;
    
    ///You're using the wrong medium type
    pub const EMEDIUMTYPE: ::c_int = 183;
    
    ///The required key is not available
    pub const ENOKEY: ::c_int = 184;
    
    ///The key you're using has expired
    pub const EKEYEXPIRED: ::c_int = 185;
    
    ///The key you're using has been revoked
    pub const EKEYREVOKED: ::c_int = 186;
    
    ///The key you're using was rejected by the service you're trying to access
    pub const EKEYREJECTED: ::c_int = 187;
    
    ///Symbol did not exist in executable
    pub const ENOSYM: ::c_int = 215;
    
    ///Socket operation attempted on an object that is not a socket
    pub const ENOTSOCK: ::c_int = 216;
    
    ///Destination address required
    pub const EDESTADDRREQ: ::c_int = 217;
    
    ///Message is too long
    pub const EMSGSIZE: ::c_int = 218;
    
    ///Protocol type is wrong for this socket
    pub const EPROTOTYPE: ::c_int = 219;
    
    ///Protocol is not available
    pub const ENOPROTOOPT: ::c_int = 220;
    
    ///Protocol is not supported
    pub const EPROTONOSUPPORT: ::c_int = 221;
    
    ///Socket type is not supported
    ///
    ///What?
    pub const ESOCKTNOSUPPORT: ::c_int = 222;
    
    ///Operation not supported on transport endpoint
    pub const EOPNOTSUPP: ::c_int = 223;
    
    ///Protocol family is not supported
    pub const EPFNOSUPPORT: ::c_int = 224;
    
    ///Address family is not supported by this protocol
    pub const EAFNOSUPPORT: ::c_int = 225;
    
    ///Address is already in use
    pub const EADDRINUSE: ::c_int = 226;
    
    ///Cannot assign the requested address
    pub const EADDRNOTAVAIL: ::c_int = 227;
    
    ///Network is down
    pub const ENETDOWN: ::c_int = 228;
    
    ///Network is unreachable :(
    pub const ENETUNREACH: ::c_int = 229;
    
    ///Network dropped connection because of a reset
    pub const ENETRESET: ::c_int = 230;
    
    ///Software caused the connection to abort
    pub const ECONNABORTED: ::c_int = 231;
    
    ///Connection was reset by a peer
    pub const ECONNRESET: ::c_int = 232;
    
    ///No buffer space is available
    pub const ENOBUFS: ::c_int = 233;
    
    ///Transport endpoint is already connected
    pub const EISCONN: ::c_int = 234;
    
    ///Transport endpoint is not connected
    pub const ENOTCONN: ::c_int = 235;
    
    ///Cannot send after the transport endpoint is shut down
    pub const ESHUTDOWN: ::c_int = 236;
    
    ///Too many references; Cannot splice
    pub const ETOOMANYREFS: ::c_int = 237;
    
    ///Connection timed out
    pub const ETIMEDOUT: ::c_int = 238;
    
    ///Connection was refused
    pub const ECONNREFUSED: ::c_int = 239;
    
    ///Connection was refused, for HP's NFS. TODO: Test this!
    pub const EREFUSED: ::c_int = ECONNREFUSED;
    
    ///Connection was released by the remote peer
    pub const EREMOTERELEASE: ::c_int = 240;
    
    ///The host is down! D:
    pub const EHOSTDOWN: ::c_int = 241;
    
    ///There is no usable route to the host
    pub const EHOSTUNREACH: ::c_int = 242;
    
    
    
    ///Operation is already in progress. Be patient!
    pub const EALREADY: ::c_int = 244;
    
    ///Operation is in progress
    pub const EINPROGRESS: ::c_int = 245;
    
    ///Operation would block. You should try again?
    pub const EWOULDBLOCK: ::c_int = ::EAGAIN;
    
    ///Directory is not empty.
    pub const ENOTEMPTY: ::c_int = 247;
    
    ///File name is too long. (Hah!)
    pub const ENAMETOOLONG: ::c_int = 248;
    
    ///Too many symbolic links encountered. This is probably an infinite loop
    pub const ELOOP: ::c_int = 249;
    
    ///Invalid system call number
    ///
    ///This is special. Arch syscall entry code returns -ENOSYS if 
    ///users try to call a syscall that doesn't exist.
    pub const ENOSYS: ::c_int = 251;
    
    ///Function not implemented
    pub const ENOTSUP: ::c_int = 252;
    
    ///The operation was cancelled
    pub const ECANCELED: ::c_int = 253;
    
    ///The operation was cancelled, but at least it was spelled correctly!
    pub const ECANCELLED: ::c_int = ECANCELED;
    
    ///Your mutex's owner died. 
    pub const EOWNERDEAD: ::c_int = 254;
    
    ///The state of your mutex is not recoverable
    pub const ENOTRECOVERABLE: ::c_int = 255;
    
    
    
    ///Operation is not possible due to RF-kill
    pub const ERFKILL: ::c_int = 256;
    
    
    
    ///The memory page has a hardware error
    pub const EHWPOISON: ::c_int = 257;
    

}

#[cfg(target_arch = "parisc")]
pub use self::errno_parisc::*;

#[cfg(target_arch = "sparc")]
pub mod errno_sparc {
    
    ///Operation would block. You should try again?
    pub const EWOULDBLOCK: ::c_int = ::EAGAIN;
    
    ///Operation is in progress
    pub const EINPROGRESS: ::c_int = 36;
    
    ///Operation is already in progress. Be patient!
    pub const EALREADY: ::c_int = 37;
    
    ///Socket operation attempted on an object that is not a socket
    pub const ENOTSOCK: ::c_int = 38;
    
    ///Destination address required
    pub const EDESTADDRREQ: ::c_int = 39;
    
    ///Message is too long
    pub const EMSGSIZE: ::c_int = 40;
    
    ///Protocol type is wrong for this socket
    pub const EPROTOTYPE: ::c_int = 41;
    
    ///Protocol is not available
    pub const ENOPROTOOPT: ::c_int = 42;
    
    ///Protocol is not supported
    pub const EPROTONOSUPPORT: ::c_int = 43;
    
    ///Socket type is not supported
    ///
    ///What?
    pub const ESOCKTNOSUPPORT: ::c_int = 44;
    
    ///Operation not supported on transport endpoint
    pub const EOPNOTSUPP: ::c_int = 45;
    
    ///Protocol family is not supported
    pub const EPFNOSUPPORT: ::c_int = 46;
    
    ///Address family is not supported by this protocol
    pub const EAFNOSUPPORT: ::c_int = 47;
    
    ///Address is already in use
    pub const EADDRINUSE: ::c_int = 48;
    
    ///Cannot assign the requested address
    pub const EADDRNOTAVAIL: ::c_int = 49;
    
    ///Network is down
    pub const ENETDOWN: ::c_int = 50;
    
    ///Network is unreachable :(
    pub const ENETUNREACH: ::c_int = 51;
    
    ///Network dropped connection because of a reset
    pub const ENETRESET: ::c_int = 52;
    
    ///Software caused the connection to abort
    pub const ECONNABORTED: ::c_int = 53;
    
    ///Connection was reset by a peer
    pub const ECONNRESET: ::c_int = 54;
    
    ///No buffer space is available
    pub const ENOBUFS: ::c_int = 55;
    
    ///Transport endpoint is already connected
    pub const EISCONN: ::c_int = 56;
    
    ///Transport endpoint is not connected
    pub const ENOTCONN: ::c_int = 57;
    
    ///Cannot send after the transport endpoint is shut down
    pub const ESHUTDOWN: ::c_int = 58;
    
    ///Too many references; Cannot splice
    pub const ETOOMANYREFS: ::c_int = 59;
    
    ///Connection timed out
    pub const ETIMEDOUT: ::c_int = 60;
    
    ///Connection was refused
    pub const ECONNREFUSED: ::c_int = 61;
    
    
    
    ///Too many symbolic links encountered. This is probably an infinite loop
    pub const ELOOP: ::c_int = 62;
    
    ///File name is too long. (Hah!)
    pub const ENAMETOOLONG: ::c_int = 63;
    
    ///The host is down! D:
    pub const EHOSTDOWN: ::c_int = 64;
    
    ///There is no usable route to the host
    pub const EHOSTUNREACH: ::c_int = 65;
    
    ///Directory is not empty.
    pub const ENOTEMPTY: ::c_int = 66;
    
    ///SunOS: Too many processes
    pub const EPROCLIM: ::c_int = 67;
    
    ///Too many users
    pub const EUSERS: ::c_int = 68;
    
    ///You exceeded your quota
    pub const EDQUOT: ::c_int = 69;
    
    ///The file handle is stale.
    pub const ESTALE: ::c_int = 70;
    
    ///Object is remote when it is required to be local
    pub const EREMOTE: ::c_int = 71;
    
    ///Device is not a stream
    pub const ENOSTR: ::c_int = 72;
    
    ///Timer expired
    pub const ETIME: ::c_int = 73;
    
    ///Out of streams resources
    ///
    ///Should this be stream's? I'm not sure about the context. Is it the resources of the stream, or resources of the system's supply of streams?
    pub const ENOSR: ::c_int = 74;
    
    ///No message of the desired type
    pub const ENOMSG: ::c_int = 75;
    
    ///Message is not a data message
    pub const EBADMSG: ::c_int = 76;
    
    ///Identifier was removed
    pub const EIDRM: ::c_int = 77;
    
    ///Resource deadlock would occur
    pub const EDEADLK: ::c_int = 78;
    
    ///No record locks available
    pub const ENOLCK: ::c_int = 79;
    
    ///Requested machine was not on the network
    pub const ENONET: ::c_int = 80;
    
    ///SunOS: Too many levels of remote in path
    pub const ERREMOTE: ::c_int = 81;
    
    ///Link has been severed
    pub const ENOLINK: ::c_int = 82;
    
    ///Error in advertising
    pub const EADV: ::c_int = 83;
    
    ///Srmount error
    pub const ESRMNT: ::c_int = 84;
    
    ///Communications error on send
    pub const ECOMM: ::c_int = 85;
    
    ///Protocol error
    pub const EPROTO: ::c_int = 86;
    
    ///A multihop was attempted
    pub const EMULTIHOP: ::c_int = 87;
    
    ///A RFS-specific error occurred
    pub const EDOTDOT: ::c_int = 88;
    
    ///The remote address changed.
    ///
    ///How does this happen? A change from wifi to 4g, etc?
    pub const EREMCHG: ::c_int = 89;
    
    ///Invalid system call number
    ///
    ///This is special. Arch syscall entry code returns -ENOSYS if 
    ///users try to call a syscall that doesn't exist.
    pub const ENOSYS: ::c_int = 90;
    
    
    
    ///Streams pipe error
    pub const ESTRPIPE: ::c_int = 91;
    
    ///Value is too large for the defined data type
    pub const EOVERFLOW: ::c_int = 92;
    
    ///File descriptor is in a bad state
    ///
    ///Huh. I haven't seen this before
    pub const EBADFD: ::c_int = 93;
    
    ///Channel number is out of acceptable range
    pub const ECHRNG: ::c_int = 94;
    
    ///Level 2 is not synchronized
    pub const EL2NSYNC: ::c_int = 95;
    
    ///Level 3 was halted
    pub const EL3HLT: ::c_int = 96;
    
    ///Level 3 was reset
    pub const EL3RST: ::c_int = 97;

    ///Link number is out of range
    ///
    ///How do you do this?
    pub const ELNRNG: ::c_int = 98;
    
    ///Protocol driver not attached
    pub const EUNATCH: ::c_int = 99;
    
    ///No CSI structure is available
    pub const ENOCSI: ::c_int = 100;
    
    ///Level 2 was halted
    pub const EL2HLT: ::c_int = 101;
    
    ///Invalid exchange
    pub const EBADE: ::c_int = 102;
    
    ///Invalid request descriptor
    pub const EBADR: ::c_int = 103;
    
    ///Exchange is full.
    pub const EXFULL: ::c_int = 104;
    
    ///No anode is available
    pub const ENOANO: ::c_int = 105;
    
    ///Invalid request code
    pub const EBADRQC: ::c_int = 106;
    
    ///Invalid slot
    pub const EBADSLT: ::c_int = 107;
    
    ///A deadlock would occur, again
    pub const EDEADLOCK: ::c_int = 108;
    
    ///Bad font file format
    pub const EBFONT: ::c_int = 109;
    
    ///Cannot exec a shared library directly
    pub const ELIBEXEC: ::c_int = 110;
    
    ///No data is available
    pub const ENODATA: ::c_int = 111;
    
    ///Attempted to access a shared library, only to find out it was corrupted
    pub const ELIBBAD: ::c_int = 112;
    
    ///Package is not installed
    pub const ENOPKG: ::c_int = 113;
    
    ///Can not access a needed shared library
    ///
    ///Thankfully, with Cargo, this shouldn't happen :)
    pub const ELIBACC: ::c_int = 114;
    
    ///Name is not unique on the network
    pub const ENOTUNIQ: ::c_int = 115;
    
    ///Interrupted system call, you should restart it
    pub const ERESTART: ::c_int = 116;
    
    ///The structure needs cleaning
    pub const EUCLEAN: ::c_int = 117;
    
    ///This is not a XENIX named type file
    pub const ENOTNAM: ::c_int = 118;
    
    ///No XENIX semaphores are available
    pub const ENAVAIL: ::c_int = 119;
    
    ///This is a named type file
    pub const EISNAM: ::c_int = 120;
    
    ///A remote I/O error occurred
    pub const EREMOTEIO: ::c_int = 121;
    
    ///Illegal byte sequence
    pub const EILSEQ: ::c_int = 122;
    
    ///Attempting to link in too many shared libraries
    pub const ELIBMAX: ::c_int = 123;
    
    ///.lib section in a.out is corrupted
    pub const ELIBSCN: ::c_int = 124;
    
    
    
    ///No medium was found
    pub const ENOMEDIUM: ::c_int = 125;
    
    ///You're using the wrong medium type
    pub const EMEDIUMTYPE: ::c_int = 126;
    
    ///The operation was cancelled
    pub const ECANCELED: ::c_int = 127;
    
    ///The required key is not available
    pub const ENOKEY: ::c_int = 128;
    
    ///The key you're using has expired
    pub const EKEYEXPIRED: ::c_int = 129;
    
    ///The key you're using has been revoked
    pub const EKEYREVOKED: ::c_int = 130;
    
    ///The key you're using was rejected by the service you're trying to access
    pub const EKEYREJECTED: ::c_int = 131;
    
    
    
    ///Your mutex's owner died. 
    pub const EOWNERDEAD: ::c_int = 132;
    
    ///The state of your mutex is not recoverable
    pub const ENOTRECOVERABLE: ::c_int = 133;
    
    
    
    ///Operation is not possible due to RF-kill
    pub const ERFKILL: ::c_int = 134;
    
    
    
    ///The memory page has a hardware error
    pub const EHWPOISON: ::c_int = 135;
    

}

#[cfg(target_arch = "sparc")]
pub use self::errno_sparc::*;
