inp = """\
name: 1, desc: "object ID"
name: 2, desc: "x pos"
name: 3, desc: "y pos"
name: 4, desc: "is flipped horizontally?"
name: 5, desc: "is flipped vertically?"
name: 6, desc: "rotation"
name: 7, desc: "Red"
name: 8, desc: "Green"
name: 9, desc: "Blue"
name: 10, desc: "Fade time / chance to trigger group 1"
name: 11, desc: "Touch triggerable"
name: 15, desc: "Using player colour 1"
name: 16, desc: "Using player colour 2"
name: 17, desc: "Blending enabled"
name: 23, desc: "Colour channel"
name: 28, desc: "Move units x"
name: 29, desc: "Move units y"
name: 30, desc: "Move easing"
name: 31, desc: "Base64-encoded text"
name: 35, desc: "Opacity"
name: 36, desc: "Is active trigger?"
name: 45, desc: "Pulse fade in time"
name: 46, desc: "Pulse hold time"
name: 47, desc: "Pulse fade out time"
name: 49, desc: "Copy colour specs"
name: 50, desc: "Copy colour from channel"
name: 51, desc: "Target group/item/channel"
name: 56, desc: "Activate group"
name: 58, desc: "Follow player's x movement"
name: 59, desc: "Follow player's y movement"
name: 60, desc: "Copy opacity"
name: 62, desc: "Spawn triggerable"
name: 71, desc: "Target group 2"
name: 75, desc: "Shake strength"
name: 80, desc: "Group/item 1"
name: 84, desc: "Shake interval"
name: 85, desc: "Easing rate"
name: 86, desc: "Exclusive pulse mode"
name: 87, desc: "Multitriggerable"
name: 94, desc: "Dynamic block?"
name: 95, desc: "Group/item 2"
name: 99, desc: "Multi activate"
name: 100, desc: "Target move mode"
name: 101, desc: "Target move mode axis lock"
name: 120, desc: "Timewarp amount"
name: 128, desc: "X scale"
name: 129, desc: "Y scale"
name: 138, desc: "Controlling player 1"
name: 141, desc: "Follow camera's x movement"
name: 142, desc: "Follow camera's y movement"
name: 143, desc: "X movement multiplier"
name: 144, desc: "Y movement multiplier"
name: 148, desc: "Gravity"
name: 200, desc: "Controlling player 2"
name: 201, desc: "Controlling target player"
name: 210, desc: "No legacy HSV"
name: 217, desc: "Enter/Exit transition config"
name: 344, desc: "Target transition channel"
name: 371, desc: "Camera zoom"
name: 392, desc: "Song ID"
name: 393, desc: "Small step"
name: 394, desc: "Directional move mode"
name: 395, desc: "Center group id"
name: 397, desc: "Dynamic move"
name: 399, desc: "Prep?"
name: 400, desc: "Load Prep?"
name: 404, desc: "Song speed"
name: 406, desc: "Song volume"
name: 408, desc: "Start offset in ms"
name: 409, desc: "Fade in time in ms"
name: 410, desc: "End offset in ms"
name: 411, desc: "Fade out time in ms"
name: 413, desc: "Loop song?"
name: 432, desc: "Song channel"
name: 445, desc: "Claim touch?"
name: 460, desc: "No end effects?"
name: 461, desc: "Instant end?"
name: 467, desc: "No end sound effects?"
name: 472, desc: "Stop time counter?"
name: 473, desc: "Target time for event"
name: 475, desc: "Multiactivatable time event"
name: 476, desc: "First item type"
name: 477, desc: "Second item type"
name: 479, desc: "Modifier"
name: 480, desc: "Left operator"
name: 481, desc: "Right operator"
name: 482, desc: "Compare operator"
name: 483, desc: "Second modifier"
name: 484, desc: "Tolerance"
name: 485, desc: "Left round mode"
name: 486, desc: "Right round mode"
name: 491, desc: "Set persistent item"
name: 492, desc: "Target all persistent items"
name: 493, desc: "Reset item to 0"
name: 494, desc: "Timer"
name: 504, desc: "Spawn only"
name: 506, desc: "Camera guide preview opacity"
name: 540, desc: "Stop player jump"
name: 541, desc: "Stop player movement"
name: 542, desc: "Stop player rotation"
name: 543, desc: "Stop player sliding"
name: 544, desc: "Silent move"
name: 547, desc: "X offset of spawned particles"
name: 548, desc: "Y offset of spawned particles"
name: 549, desc: "X offset variation of spawned particles"
name: 550, desc: "Y offset variation of spawned particles"
name: 551, desc: "Match rotation of spawned particles?"
name: 552, desc: "Rotation of spawned particles"
name: 553, desc: "Rotation variation of spawned particles"
name: 554, desc: "Scale of spawned particles"
name: 555, desc: "Scale variation of spawned particles"
name: 578, desc: "Left sign mode"
name: 579, desc: "Right sign mode"
name: 595, desc: "Don't stop song on death"
name: 10004, desc: "Starting speed"
name: 10002, desc: "Starting gamemode"
name: 10003, desc: "Starting in mini mode?"
name: 10008, desc: "Starting in dual mode?"
name: 10021, desc: "Is disabled?"
name: 10028, desc: "Starting in mirror mode?"
name: 10029, desc: "Rotate gameplay?"
name: 10020, desc: "Reverse gameplay?"
name: 10019, desc: "Target order"
name: 10026, desc: "Target channel"
name: 10035, desc: "Reset camera?\""""

char_blacklist = "+'/?\""

def objects(line):
    split = line.strip("(),+").strip().split(", ")

    id, desc = split

    for ch in char_blacklist:
        desc = desc.replace(ch, "")

    while "  " in desc:
        desc = desc.replace("  ", " ")
    desc = desc[1:-1].replace(" ", "_").upper()
    print(f"pub const {desc}: i32 = {id};")

def properties(line):
    split = line.strip('"').strip().split(", ")

    name, desc = split
    name = name.split("name: ")[1]
    desc = desc.split("desc: ")[1]

    for ch in char_blacklist:
        desc = desc.replace(ch, "")

    while "  " in desc:
        desc = desc.replace("  ", " ")
    desc = desc.replace(" ", "_").upper()
    print(f"pub const PROPERTY_{desc}: i32 = {name};")

for line in inp.split("\n"):
    # objects(line)
    properties(line)
