/*! Actions which do not only call other base plugins. */

mod configure_nix_daemon_service;
mod create_directory;
mod create_file;
mod create_group;
mod create_or_append_file;
mod create_user;
mod fetch_nix;
mod move_unpacked_nix;
mod place_channel_configuration;
mod place_nix_configuration;
mod setup_default_profile;
mod start_systemd_unit;

pub use configure_nix_daemon_service::{
    ConfigureNixDaemonService, ConfigureNixDaemonServiceError,
};
pub use create_directory::{CreateDirectory, CreateDirectoryError};
pub use create_file::{CreateFile, CreateFileError};
pub use create_group::{CreateGroup, CreateGroupError};
pub use create_or_append_file::{CreateOrAppendFile, CreateOrAppendFileError};
pub use create_user::{CreateUser, CreateUserError};
pub use fetch_nix::{FetchNix, FetchNixError};
pub use move_unpacked_nix::{MoveUnpackedNix, MoveUnpackedNixError};
pub use place_channel_configuration::{
    PlaceChannelConfiguration, PlaceChannelConfigurationError,
};
pub use place_nix_configuration::{PlaceNixConfiguration, PlaceNixConfigurationError};
pub use setup_default_profile::{SetupDefaultProfile, SetupDefaultProfileError};
pub use start_systemd_unit::{StartSystemdUnit, StartSystemdUnitError};
