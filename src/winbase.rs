// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! This module defines the 32-Bit Windows Base APIs
//99
pub const FILE_BEGIN: ::DWORD = 0;
pub const FILE_CURRENT: ::DWORD = 1;
pub const FILE_END: ::DWORD = 2;
//123
pub const FILE_FLAG_WRITE_THROUGH: ::DWORD = 0x80000000;
pub const FILE_FLAG_OVERLAPPED: ::DWORD = 0x40000000;
pub const FILE_FLAG_NO_BUFFERING: ::DWORD = 0x20000000;
pub const FILE_FLAG_RANDOM_ACCESS: ::DWORD = 0x10000000;
pub const FILE_FLAG_SEQUENTIAL_SCAN: ::DWORD = 0x08000000;
pub const FILE_FLAG_DELETE_ON_CLOSE: ::DWORD = 0x04000000;
pub const FILE_FLAG_BACKUP_SEMANTICS: ::DWORD = 0x02000000;
pub const FILE_FLAG_POSIX_SEMANTICS: ::DWORD = 0x01000000;
pub const FILE_FLAG_SESSION_AWARE: ::DWORD = 0x00800000;
pub const FILE_FLAG_OPEN_REPARSE_POINT: ::DWORD = 0x00200000;
pub const FILE_FLAG_OPEN_NO_RECALL: ::DWORD = 0x00100000;
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: ::DWORD = 0x00080000;
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: ::DWORD = 0x00040000;
pub const PROGRESS_CONTINUE: ::DWORD = 0;
pub const PROGRESS_CANCEL: ::DWORD = 1;
pub const PROGRESS_STOP: ::DWORD = 2;
pub const PROGRESS_QUIET: ::DWORD = 3;
pub const CALLBACK_CHUNK_FINISHED: ::DWORD = 0x00000000;
pub const CALLBACK_STREAM_SWITCH: ::DWORD = 0x00000001;
pub const COPY_FILE_FAIL_IF_EXISTS: ::DWORD = 0x00000001;
pub const COPY_FILE_RESTARTABLE: ::DWORD = 0x00000002;
pub const COPY_FILE_OPEN_SOURCE_FOR_WRITE: ::DWORD = 0x00000004;
pub const COPY_FILE_ALLOW_DECRYPTED_DESTINATION: ::DWORD = 0x00000008;
pub const COPY_FILE_COPY_SYMLINK: ::DWORD = 0x00000800;
pub const COPY_FILE_NO_BUFFERING: ::DWORD = 0x00001000;
pub const COPY_FILE_REQUEST_SECURITY_PRIVILEGES: ::DWORD = 0x00002000;
pub const COPY_FILE_RESUME_FROM_PAUSE: ::DWORD = 0x00004000;
pub const COPY_FILE_NO_OFFLOAD: ::DWORD = 0x00040000;
pub const REPLACEFILE_WRITE_THROUGH: ::DWORD = 0x00000001;
pub const REPLACEFILE_IGNORE_MERGE_ERRORS: ::DWORD = 0x00000002;
pub const REPLACEFILE_IGNORE_ACL_ERRORS: ::DWORD = 0x00000004;
pub const PIPE_ACCESS_INBOUND: ::DWORD = 0x00000001;
pub const PIPE_ACCESS_OUTBOUND: ::DWORD = 0x00000002;
pub const PIPE_ACCESS_DUPLEX: ::DWORD = 0x00000003;
pub const PIPE_CLIENT_END: ::DWORD = 0x00000000;
pub const PIPE_SERVER_END: ::DWORD = 0x00000001;
pub const PIPE_WAIT: ::DWORD = 0x00000000;
pub const PIPE_NOWAIT: ::DWORD = 0x00000001;
pub const PIPE_READMODE_BYTE: ::DWORD = 0x00000000;
pub const PIPE_READMODE_MESSAGE: ::DWORD = 0x00000002;
pub const PIPE_TYPE_BYTE: ::DWORD = 0x00000000;
pub const PIPE_TYPE_MESSAGE: ::DWORD = 0x00000004;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: ::DWORD = 0x00000000;
pub const PIPE_REJECT_REMOTE_CLIENTS: ::DWORD = 0x00000008;
pub const PIPE_UNLIMITED_INSTANCES: ::DWORD = 255;
//268
pub const SECURITY_CONTEXT_TRACKING: ::DWORD = 0x00040000;
pub const SECURITY_EFFECTIVE_ONLY: ::DWORD = 0x00080000;
pub const SECURITY_SQOS_PRESENT: ::DWORD = 0x00100000;
pub const SECURITY_VALID_SQOS_FLAGS: ::DWORD = 0x001F0000;
//687
pub const STD_INPUT_HANDLE: ::DWORD = -10;
pub const STD_OUTPUT_HANDLE: ::DWORD = -11;
pub const STD_ERROR_HANDLE: ::DWORD = -12;
//5454
pub const MOVEFILE_REPLACE_EXISTING: ::DWORD = 0x00000001;
pub const MOVEFILE_COPY_ALLOWED: ::DWORD = 0x00000002;
pub const MOVEFILE_DELAY_UNTIL_REBOOT: ::DWORD = 0x00000004;
pub const MOVEFILE_WRITE_THROUGH: ::DWORD = 0x00000008;
pub const MOVEFILE_CREATE_HARDLINK: ::DWORD = 0x00000010;
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: ::DWORD = 0x00000020;