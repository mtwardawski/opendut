use leptos::{component, create_read_slice, create_slice, event_target_value, IntoView, MaybeSignal, RwSignal, SignalGet, SignalGetUntracked, SignalWith, view};

use opendut_types::topology::{DeviceDescription, DeviceId, DeviceName, IllegalDeviceName};

use crate::components::{ButtonColor, ButtonSize, ButtonState, ConfirmationButton, FontAwesomeIcon, IconButton, ReadOnlyInput, Toggled, UserInput, UserInputValue, UserTextarea};
use crate::peers::configurator::types::{EMPTY_DEVICE_NAME_ERROR_MESSAGE, UserDeviceConfiguration, UserPeerConfiguration};

#[component]
pub fn DevicePanel<OnDeleteFn>(
    peer_configuration: RwSignal<UserPeerConfiguration>,
    device_configuration: RwSignal<UserDeviceConfiguration>,
    on_delete: OnDeleteFn
) -> impl IntoView
where
    OnDeleteFn: Fn(DeviceId) + 'static
{
    let device_id_string = MaybeSignal::derive(move || device_configuration.get().id.to_string());
    let is_collapsed = move || device_configuration.get().is_collapsed;

    view! {
        <div class="panel is-light">
            <DevicePanelHeading device_configuration on_delete />
            <div
                class="panel-block"
                class=("is-hidden", is_collapsed)
            >
                <div class="container">
                    <ReadOnlyInput label="ID" value=device_id_string />
                    <DeviceNameInput device_configuration />
                    <DeviceInterfaceInput peer_configuration device_configuration />
                    <DeviceDescriptionInput device_configuration />
                </div>
            </div>
        </div>
    }
}

#[component]
fn DevicePanelHeading<OnDeleteFn>(
    device_configuration: RwSignal<UserDeviceConfiguration>,
    on_delete: OnDeleteFn
) -> impl IntoView
where
    OnDeleteFn: Fn(DeviceId) + 'static
{
    let (is_collapsed, set_is_collapsed) = create_slice(device_configuration,
        move |device_configuration| {
            device_configuration.is_collapsed
        },
        move |device_configuration, value| {
            device_configuration.is_collapsed = value;
        }
    );

    let collapse_button_icon = is_collapsed.derive_toggled(FontAwesomeIcon::ChevronDown, FontAwesomeIcon::ChevronUp);

    let device_name = create_read_slice(device_configuration,
        |device_configuration| {
            match device_configuration.name {
                UserInputValue::Left(_) => String::from(""),
                UserInputValue::Right(ref value) => value.to_owned(),
                UserInputValue::Both(_, ref value) => value.to_owned()
            }
        }
    );

    view! {
        <div class="panel-heading px-2 py-3">
            <div class="is-flex is-justify-content-space-between is-align-items-center">
                <div>
                    <span class="pr-1">
                        <IconButton
                            icon=collapse_button_icon
                            color=ButtonColor::Light
                            size=ButtonSize::Small
                            state=ButtonState::Enabled
                            label="Show Device Details"
                            on_action=move || set_is_collapsed.set(!is_collapsed.get_untracked())
                        />
                    </span>
                    <span class="is-size-5 has-text-weight-bold">{ device_name }</span>
                </div>
                <div>
                    <ConfirmationButton
                        icon=FontAwesomeIcon::TrashCan
                        color=ButtonColor::Light
                        size=ButtonSize::Small
                        state=ButtonState::Enabled
                        label="Delete Device?"
                        on_conform=move || on_delete(device_configuration.get_untracked().id)
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn DeviceNameInput(
    device_configuration: RwSignal<UserDeviceConfiguration>,
) -> impl IntoView {

    let (getter, setter) = create_slice(device_configuration,
        |device_configuration| {
            Clone::clone(&device_configuration.name)
        },
        |device_configuration, value| {
            device_configuration.name = value;
        }
    );

    let validator = |input: String| {
        match DeviceName::try_from(input) {
            Ok(name) => {
                UserInputValue::Right(String::from(name.value()))
            }
            Err(cause) => {
                match cause {
                    IllegalDeviceName::TooShort { expected, actual, value } => {
                        if actual > 0 {
                            UserInputValue::Both(format!("A device name must be at least {} characters long.", expected), value)

                        }
                        else {
                            UserInputValue::Both(String::from(EMPTY_DEVICE_NAME_ERROR_MESSAGE), value)
                        }
                    },
                    IllegalDeviceName::TooLong { expected, value, .. } => {
                        UserInputValue::Both(format!("A device name must be at most {} characters long.", expected), value)
                    },
                    IllegalDeviceName::InvalidStartEndCharacter { value } => {
                        UserInputValue::Both("The device name starts/ends with an invalid character. \
                        Valid characters are a-z, A-Z and 0-9.".to_string(), value)
                    }
                    IllegalDeviceName::InvalidCharacter { value } => {
                        UserInputValue::Both("The device name contains invalid characters. \
                        Valid characters are a-z, A-Z, 0-9 and _-".to_string(), value)
                    }
                }
            }
        }
    };

    view! {
        <UserInput
            getter
            setter
            label="Name"
            placeholder="Device A"
            validator
        />
    }
}

#[component]
fn DeviceInterfaceInput(
    peer_configuration: RwSignal<UserPeerConfiguration>,
    device_configuration: RwSignal<UserDeviceConfiguration>,
) -> impl IntoView {

    let peer_network_interfaces = create_read_slice(peer_configuration,
        |peer_network_interfaces| {
            Clone::clone(&peer_network_interfaces.network.network_interfaces)
        }
    );

    let (getter, setter) = create_slice(device_configuration,
        |device_configuration| {
            Clone::clone(&device_configuration.interface)
        },
        |device_configuration, value| {
            device_configuration.interface = value;
        }
    );

    let value_text = getter.with(|input| match input {
        None => String::from("Select interface"),
        Some(interface) => format!("{} ({})", interface.name.name(), interface.configuration.display_name() ),
    });

    let dropdown_options = move || {
        peer_network_interfaces.with(|interfaces | {
            interfaces.iter().cloned()
                .map(|interface| {
                    let name = interface.get_untracked().name.name();
                    let interface_type = interface.get_untracked().configuration.display_name();
                    if value_text == format!("{} ({})", name, interface_type) {
                        view! {
                            <option value={name.clone()} selected>{name} " ("{interface_type}")"</option>
                        }
                    } else {
                        view! {
                            <option value={name.clone()}>{name} " ("{interface_type}")"</option>
                        }
                    }

                })
                .collect::<Vec<_>>()
        })
    };

    let configuration = move | selected_interface_name: String | {
        peer_network_interfaces.with(|interfaces| {
            interfaces.iter()
                .find(|interface| {
                    interface.get().name.name() == selected_interface_name
                })
                .map(| user_network_interface | {
                    user_network_interface.get()
                })
        })
    };

    view! {
        <div class="field">
            <label class="label">Interface</label>
            <div class="control">
                <div class="select"
                    on:change=move |ev| {
                        let target_value = event_target_value(&ev);
                        if target_value == "Select interface" {
                            setter.set(None);
                        } else {
                            setter.set(
                                configuration(target_value)
                            );
                        }
                    }>
                    <select>
                        <option>Select interface</option>
                        { dropdown_options }
                    </select>
                </div>
            </div>
        </div>
    }
}

#[component]
fn DeviceDescriptionInput(
    device_configuration: RwSignal<UserDeviceConfiguration>
) -> impl IntoView {

    let (getter, setter) = create_slice(device_configuration,
        |device_configuration| {
            Clone::clone(&device_configuration.description)
        },
        |device_configuration, value| {
            device_configuration.description = value;
        }
    );

    let validator = |input: String| {
        match DeviceDescription::try_from(Clone::clone(&input)) {
            Err(error) => {
                UserInputValue::Both(error.to_string(), input)
            }
            Ok(_) => {
                UserInputValue::Right(input)
            }
        }
    };

    view! {
        <UserTextarea
            getter=getter
            setter=setter
            label="Description"
            placeholder="Description"
            validator
        />
    }
}

