# Hadalyth-Vipo

These are my personal GDExtensions for interfacing with various services from Godot for making my streaming overlay.

## Hadalyht-Twitch Example

```gdscript

# In this example, this is a global

extends Node

const DATA_PATH : String = "res://data"
const TWITCH_PATH : String = "res://data/twitch"
const CLIENT_FILE : String = TWITCH_PATH + "/twitch_client_id.tres"
const SCOPES_FILE : String = TWITCH_PATH + "/twitch_scopes.tres"
const EVENTSUBS_FILE : String = TWITCH_PATH + "/twitch_eventsubs.tres"

var _client_id : HadalythTwitchClientId = null
var _scopes : HadalythTwitchScopes = null
var _eventsubs : HadalythTwitchEventSubs = null

@onready var hadalyth_twitch : HadalythTwitch = HadalythTwitch.new() 

func _ready() -> void:

    self.add_child(hadalyth_twitch)

    # Note : This is an example of connecting to a singal
    # You probably want to do this somewhere else

    hadalyth_twitch.recv_channel_bits_use_v1.connect(_on_recv_channel_bits_use_v1)

    # Note : You can load the data automatically or from somewhere else before calling init

    # Create data and twitch directory
    if not DirAccess.dir_exists_absolute(DATA_PATH):
        DirAccess.make_dir_absolute(DATA_PATH)
    if not DirAccess.dir_exists_absolute(TWITCH_PATH):
        DirAccess.make_dir_absolute(TWITCH_PATH)

    # Autoload resources or create if they don't exist
    if not FileAccess.file_exists(CLIENT_FILE):
        _client_id = HadalythTwitchClientId.new()
        ResourceSaver.save(_client_id, CLIENT_FILE)
    _client_id = ResourceLoader.load(CLIENT_FILE, "HadalythTwitchClientId")
    hadalyth_twitch.set_client_id(_client_id)

    # Autoload resources or create if they don't exist
    if not FileAccess.file_exists(SCOPES_FILE):
        _scopes = HadalythTwitchScopes.new()
        ResourceSaver.save(_scopes, SCOPES_FILE)
    _scopes = ResourceLoader.load(SCOPES_FILE, "HadalythTwitchScopes")
    hadalyth_twitch.set_scopes(_scopes)

    # Autoload resources or create if they don't exist
    if not FileAccess.file_exists(EVENTSUBS_FILE):
        _eventsubs = HadalythTwitchEventSubs.new()
        ResourceSaver.save(_eventsubs, EVENTSUBS_FILE)
    _eventsubs = ResourceLoader.load(EVENTSUBS_FILE, "HadalythTwitchEventSubs")
    hadalyth_twitch.set_eventsubs(_eventsubs)

    
    init_twitch()


## Start the init step
func init_twitch() -> void:

    # Note : Call this from somewhere in your code
    # probably a menu and wait for it to finish

    hadalyth_twitch.init_twitch()
    
    # Note : You can wait for the different init steps to check if anything fails
    # Also do this from where in your code

    var device_user_token_status : bool = await hadalyth_twitch.device_user_token_status
    if not device_user_token_status:
        push_error("DEVICE TOKEN FAILED")
        return

    var twitch_socket_status : bool = await hadalyth_twitch.twitch_socket_status
    if not twitch_socket_status:
        push_error("TWITCH SOCKET FAILED")
        return


## Kill the any running twitch components
## Lingering socket events may come through for a few seconds until the stream is exhausted
## Or the connection is stopped
func kill_twitch() -> void:
    hadalyth_twitch.kill_twitch()


func _on_recv_channel_bits_use_v1(
    bits: int, 
    broadcaster: Broadcaster, 
    bits_custom_power_up: BitsCustomPowerUp, 
    message: Message, 
    bits_type: int, 
    user: User
) -> void:
    
    print(bits)
    print(broadcaster.user_id)
    print(broadcaster.user_login)
    print(broadcaster.user_name)
    print(bits_custom_power_up.title)
    print(bits_custom_power_up.reward_id)
    print(message.text)
    for fragment : Fragment in message.fragments:
        print(fragment.text)
        if is_instance_valid(fragment.cheermote):
            print(fragment.cheermote.bits)
            print(fragment.cheermote.prefix)
            print(fragment.cheermote.tier)
        if is_instance_valid(fragment.emote):
            print(fragment.emote.id)
            print(fragment.emote.emote_set_id)
            print(fragment.emote.owner_id)
        if is_instance_valid(fragment.mention):
            print(fragment.mention.user.user_id)
            print(fragment.mention.user.user_login)
            print(fragment.mention.user.user_name)
    print(bits_type)
    print(user.user_id)
    print(user.user_login)
    print(user.user_name)

```