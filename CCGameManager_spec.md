# CCGameManager structure specification
Reference: https://boomlings.dev/resources/client/gamesave
# 1. outline
## 1.1. Gamesave Key structure
| Key | Type | Description |
|---|---|---|
| bgVolume | Float | any value between 0 - 1 |
| sfxVolume | Float | any value between 0 - 1 |
| playerUDID | UDID | The UDID of the user |
| playerName | String | The In-game Name of the player |
| playerUserID | Integer | The userID of the player |
| playerFrame | Icon | The ID of the player's cube |
| playerShip | Icon | The ID of the player's ship |
| playerBall | Icon | The ID of the player's ball |
| playerBird | Icon | The ID of the player's UFO |
| playerDart | Icon | The ID of the player's wave |
| playerRobot | Icon | The ID of the player's robot |
| playerSpider | Icon | The ID of the player's spider |
| playerSwing | Icon | The ID of the player's swing |
| playerColor | Icon | The ID of the player's primary color |
| playerColor2 | Icon | The ID of the player's secondary color |
| playerColor3 | Icon | The ID of the player's glow color, -1 if same as secondary color |
| playerStreak | Icon | The ID of the player's trail |
| playerShipStreak | Icon | The ID of the player's ship streak |
| playerDeathEffect | Icon | The ID of the player's death effect |
| playerJetpack | Icon | The ID of the player's jetpack |
| playerIconType | Icon | The Index of the player's IconType |
| playerGlow | Bool | if Glow is enabled or not |
| secretNumber | Integer | the answer to cod3breaker in the vault of secrets |
| hasRP | Integer | The player's moderator status, presumably 1 for mod and 2 for elder mod |
| hasDRP | Bool | If the player has demon rating privileges (leaderboard mods) |
| valueKeeper | Dictionary | Dictionary filled with GV values you have enabled |
| unlockValueKeeper | Dictionary | Dictionary of Unlockable GV values |
| customObjectDict | Dictionary | Dictionary of object data of custom objects saved |
| reportedAchievements | Dictionary | contains a dictonary of all reported achievements on your account |
| showSongMarkers | Bool | if showSongMarkers is enabled |
| showProgressBar | Bool | if showProgressBar is enabled |
| performanceMode | Bool | If the global Low Detail Mode in settings is enabled |
| clickedGarage | Bool | if Garage has been clicked |
| clickedEditor | Bool | if Editor has been clicked |
| clickedPractice | Bool | if Practice mode button has been clicked |
| showedEditorGuide | Bool | if EditorGuide has been clicked |
| showedLowDetailDialog | Bool | if low detail mode has been has been clicked |
| showedRateStarDialog | Bool | Whether the difficulty rating explanation was shown |
| bootups | integer | The ammount of times you have opened Geometry Dash |
| hasRatedGame | Bool | if you have rated the game |
| binaryVersion | integer | The Games Binary Version |
| resolution | resolution | The games resolution |
| texQuality | integer | how high the text quality is (0 for Auto, 1 for Low, 2 for Medium and 3 for High) |
| timeOffset | integer | music offset in milliseconds |
| customFPSTarget | float | the FPS target value |
| customMenuSongID | integer | The song ID chosen as the menu music |
| customPracticeSongID | integer | The song ID chosen as the practice music |
| dpad01 | Platformer UI | Platformer UI configuration #1 |
| dpad02 | Platformer UI | Platformer UI configuration #2 |
| dpad03 | Platformer UI | Platformer UI configuration #3 |
| dpad04 | Platformer UI | Platformer UI configuration #4, however it's missing the last 5 options |
| dpad05 | Platformer UI | Platformer UI configuration #5, however it's missing the last 5 options |
| dpadLayout01 | Platformer UI | Platformer UI save #1 |
| dpadLayout02 | Platformer UI | Platformer UI save #2 |
| dpadLayout03 | Platformer UI | Platformer UI save #3 |
| dpadLayoutDual01 | Platformer UI | 2 platformer UIs separated by ;, then 2 gamepad placements, also separated by ; |
| dpadLayoutDual02 | Platformer UI | 2 platformer UIs separated by ;, then 2 gamepad placements, also separated by ; |
| dpadLayoutDual03 | Platformer UI | 2 platformer UIs separated by ;, then 2 gamepad placements, also separated by ; |
| practiceOpacity | float | The opacity of the practice UI, from 0 to 1 |
| practicePosX | float | The X position of the practice UI |
| practicePosY | float | The Y position of the practice UI |
| newsLastReadID | integer | The ID of the news post you last saw |
| recentEmotes | string | 16 of your most recently used emotes, separated by commas |
| editorSFXGlobalVolume | float | The volume for sound packs |
| scrollSensitivity | float | The Scroll option in the "Other" settings category |
| specialPermKey | integer | Unknown, presumably something related to the new mod system |

## 1.2. Legacy Keys
These keys were used in old versions of the game but are now obsolete.

| Key | Type | description |
|---|---|---|
| musicEnabled | Bool | Whether music is enabled or not. Moved to bgVolume |
| fxEnabled | Bool | Whether sound effects are enabled or not. Moved to sfxVolume |
| moreGamesString | String | The contents of the More Games button. It was a list of values separated by _. The values were: gj for GD Lite, boom for Boomlings, mu for Boomlings MatchUp and mm for Memory Mastermind. The game allowed repeated  |values
| hasNewGames | Bool | Whether there are new games in the More Games button. This showed an exclamation mark icon next to the button |
| gameCenterEnabled | Bool | Whether the Game Center (iOS) was enabled. Moved to valueKeeper::gv_0034 |
| lastDay | Integer | Presumably the day when you last played (0-indexed) |
| lastMonth | Integer | Presumably the month when you last played (0-indexed) |
| clickedName | Bool | Whether you clicked on your name in the icon kit to change it |
| autoCheckpoints | Bool | Whether checkpoints are placed automatically in practice mode. Moved to valueKeeper::gv_0027 |
| showBPMMarkers | Bool | if BPM markers are shown |
| autoRetryLevel | Bool | If the level automatically restarts after death. Moved to valueKeeper::gv_0026 |
| recordGameplay | Bool | Presumably whether to record gameplay with Everyplay |
| showedRateDiffDialog | Bool | Whether the Rate Difficulty explanation was shown (separate from the current Rate Stars button; there used to be a separate button for rating difficulty from rating stars) |
| timeCounter | Integer | An anti-cheat measure that existed in 1.7 and 1.8 that was meant to stop save editing for stars and demons. The value was calculated like this: floor((stars + demons) * 0.5346). If, when loading the game, the number  didn't match, stars and demons would be set to 0 |
| commentSortRecent | Bool | unknown (there was no option to change the comment sorting at the time) |
| kEnableTutorial | Bool | unknown |
| showedFirstTutorial | Bool | unknown |
| firstSetup | Bool | unknown, possibly whether the save is being created for the first time |
| playerScoreValid | Bool | unknown |

## 1.3. GLM
GLM (Global Levels Manager) are various Dictionaries that contain every bit of information you need to know about online levels

| Key | Type | description |
|---|---|--|
| GLM_01 | Level | All Official Levels you have progress on are stored here |
| GLM_02 | level | Local levels - before 1.1, when they were moved from CCGameManager.dat to a new file: CCLocalLevels.dat |
| GLM_03 | Level | online levels played |
| GLM_04 | rating | Shows what levels you have rated. was removed after 1.9 |
| GLM_06 | AccountIDs | The AccountIDs of all creators you follow |
| GLM_07 | levelID | Levels played in last session |
| GLM_08 | filters | Search Filters States |
| GLM_09 | filters | Search Filters for Online Levels |
| GLM_10 | Level | Completed dailies |
| GLM_11 | Integer | Current Daily ID you are on |
| GLM_12 | likes | Something Related to likes |
| GLM_13 | levelID | All levels you submitted a rating on |
| GLM_14 | reportedLevels | A dictionary of all levels you have reported |
| GLM_15 | levelID | all Demon levels you have submitted a rating for |
| GLM_16 | Level | All the levels found in the Gauntlets that you have progress on are stored in here |
| GLM_17 | integer | Current Weekly ID you are on |
| GLM_18 | Folder | The Folder Names for saved levels |
| GLM_19 | Folder | The Folder names for Local Levels |
| GLM_20 | Templates | Your Smart Templates |
| GLM_21 | Lists | Saved lists, only existed briefly in 2.200 |
| GLM_22 | Lists | Your favourited lists |
| GLM_23 | Integer | Current Event ID you are on |
| GLM_24 | Integer | Current Daily ID on the servers |
| GLM_25 | Integer | Current Weekly ID on the servers |
| GLM_26 | Integer | Current Event ID on the servers |


### GLM_01

GLM_01 is used to store all progress the player has made on official levels

| Key | description |
|-----|-------------|
| kCEK 4 | level data |

### GLM_02

GLM_02 was used in the earliest versions of the game back when your local levels were stored in CCGameManager.dat instead of being separated to CCLocalLevels.dat, currently unused

| Key | description |
|-----|-------------|
| k_{Num} | what place the level is placed in your level list |
| kCEK 4 | level data |

### GLM_03

GLM_03 is used to store progress the player has done on all online levels

| Key | description |
|-----|-------------|
| kCEK 4 | level data |

### GLM_04

GLM_04 contains all the difficulty ratings you have given levels, currently unused

| key | value |
|-----|-------------|
| `{levelID}`| `{diffID}` | 


### GLM_06

GLM_06 contains all of the creators you follow in game

| key | value | description |
|-----|-------|-------------|
| AccountID | 1 | Doing this would place the creators levels in the followed tab |

### GLM_07

GLM_07 contains the levels you played in the last session

| Key | description |
|-----|-------------|
| levelID | accompanied with a 1 |

### GLM_08

GLM_08 are the states of search filters

| Key | Value |
|-----|-------|
| Diff0 | NA |
| Diff1 | Auto |
| Diff2 | Easy |
| Diff3 | Normal |
| Diff4 | Hard |
| Diff5 | Harder |
| Diff6 | Insane |
| Diff7 | Demon |
| Len0 | Tiny |
| Len1 | Short |
| Len2 | Medium |
| Len3 | Long |
| Len4 | XL |
| demon_filter | Demon difficulty |
| uncompleted_filter | bool |
| completed_filter | bool |
| featured_filter | bool |
| song_filter | official SongID |
| original_filter | bool |
| twoP_filter | bool |
| enable_songFilter | bool |
| customsong_filter | songID |
| nostar_filter | bool |
| coin_filter | bool |
| follow_filter | bool |
| friend_filter | bool |
| epic_filter | bool |
| star_filter | bool |

### GLM_09

GLM_09 shows all search filters available

| Key | Value |
|-----|-------|
| Diff0 | NA |
| Diff1 | Auto |
| Diff2 | Easy |
| Diff3 | Normal |
| Diff4 | Hard |
| Diff5 | Harder |
| Diff6 | Insane |
| Diff7 | Demon |
| Len0 | Tiny |
| Len1 | Short |
| Len2 | Medium |
| Len3 | Long |
| Len4 | XL |
| demon_filter | [Demon difficulty](/resources/server/level#demon-difficulty) |
| uncompleted_filter | bool |
| completed_filter | bool |
| featured_filter | bool |
| song_filter | official SongID |
| original_filter | bool |
| twoP_filter | bool |
| enable_songFilter | bool |
| customsong_filter | songID |
| nostar_filter | bool |
| coin_filter | bool |
| follow_filter | bool |
| friend_filter | bool |
| epic_filter | bool |
| star_filter | bool |

### GLM_10

GLM_10 contains every daily level you have ever had progress on

| Key | Description |
|-----|-------------|
| {timelyID} | A dictionary which contains the Encoder key for levels |
| kCEK 4 | The Encoder key which contains [level data](/resources/client/level.md)

### GLM_11

GLM_11 is the current Daily ID you are on

### GLM_12

GLM_12 contains likes given to a level

| key | Description |
|-----|-------------|
| `like_{LikeType}_{LevelID}_{like/dislike}_{ID}` | needs investigating more |

### GLM_13

GLM_13 Contains all the levelIDs for levels you have submitted a rating for

### GLM_14

GLM_14 Contains all the levelIDs of levels you have reported

### GLM_15

GLM_15 Contains all the levelIDs of Demons you have rated

### GLM_16

GLM_16 contains every Gauntlet level you have ever had progress on

| Key | Description |
|-----|-------------|
| {timelyID} | A dictionary which contains the Encoder key for levels |
| kCEK 4 | The Encoder key which contains [level data](/resources/client/level.md)

### GLM_17

GLM_17 is the current WeeklyID you are on

### GLM_18

GLM_18 is a dictionary of the Saved Levels folder for online levels

| Key | Value |
|-----|-------|
| `{folder ID}` | `{Folder Name}` |

### GLM_19

GLM_19 is a dictionary of the folders for local levels

| Key | Value |
|-----|-------|
| `{folder ID}` | `{Folder Name}` |

### GLM_20

GLM_20 is a dictionary of all the editor smart templates you have saved

| Key | description |
|-----|-------------|
| kCEK 10 | [level data](/resources/client/gamesave/template.md) |

### GLM_21

GLM_21 used to be a dictionary of all your saved lists in 2.200, but this feature has since been removed

| Key     | description                                      |
| ------- | ------------------------------------------------ |
| kCEK 12 | [level data](/resources/client/gamesave/list.md) |

### GLM_22

GLM_22 is a dictionary of all your favorited lists

| Key | description |
|-----|-------------|
| kCEK 12 | [level data](/resources/client/gamesave/list.md) |

### GLM_23

GLM_23 is the current Event ID you are on

### GLM_24

GLM_24 is the current Daily ID on the server

### GLM_25

GLM_25 is the current Weekly ID on the server

### GLM_26

GLM_26 is the current Event ID on the server

## 1.4. GS

GS values contain Information regarding certain aspects of the game

| Key | Type |
|---|---|
| GS_value | Player Stats |
| GS_completed | Completed Levels |
| GS_3 | Completed levels with coins {levelID}_{number of coins aquired from level} |
| GS_4 | Completed levels with coins {levelID}_{number of coins aquired from level} |
| GS_5 | Completed Mappacks + rewarded stars <k>pack_{MappackID}</k><s>{Stars rewarded}</s> |
| GS_6 | all Purchased Icons/Colours in the shops <k>{ListingID}</k><s>{Price}</s> |
| GS_7 | level progress <k>{levelID}</k><s>{percentage}</s> |
| GS_8 | Unused |
| GS_9 | How many stars downloaded levels give <k>{levelID}</k><s>{rewardedStars}</s> |
| GS_10 | Offical level progress <k>{ID}</k><s>{percentage}</s> |
| GS_11 | All rewards gained from Daily Chests |
| GS_12 | Contains Quests |
| GS_14 | Daily/Challenges rewards |
| GS_15 | Contains Upcoming Quests |
| GS_16 | Daily/Weekly level progress <k>{Daily/WeeklyID}</k><s>{percentage}</s> |
| GS_17 | Daily/Weekly Stars <k>{Daily/WeeklyID}</k><s>{Stars}</s> |
| GS_18 | Gauntlet Level Progress <k>{levelID}</k><s>{percentage}</s> |
| GS_19 | All the Rewards from unlocked chests in the treasure room |
| GS_20 | Demon Keys |
| GS_21 | All rewards for Completed Gauntlets + rewards for Demon quest/social medias |
| GS_22 | Rewards from Geometry Dash world |
| GS_23 | Gauntlet Level Progress <k>{levelID}</k><s>{percentage}</s> |
| GS_24 | Daily/Weekly Percentage |
| GS_25 | All the rewards from completed weekly Demons |
| GS_26 | Your active path |
| GS_27 | All the list rewards |
| GS_28 | Your enabled items (animations) |
| GS_29 | Some path bug fix (boolean) |
| GS_30 | All rewards gained from Secret Room (Wraith) chests |
| GS_31 | All rewards gained from Event Level chests |
| GS_32 | All unlocked gauntlets (currently unused) |

#### GS_value

| key | value |
|-----|-------|
| 1 | Jumps |
| 2 | Attempts |
| 3 | Total Completed Official Levels |
| 4 | Total Completed Online Levels |
| 5 | Completed Demons |
| 6 | Total Stars |
| 7 | Total Completed MapPacks |
| 8 | Secret Coins Collected |
| 9 | Destroyed Players on the menu |
| 10 | Total Liked/Disliked levels |
| 11 | Total Rated Levels |
| 12 | User coins collected |
| 13 | Total Diamonds |
| 14 | current orbs |
| 15 | Completed Daily Levels |
| 16 | Shadow Shards |
| 17 | Poison Shards |
| 18 | Fire Shards |
| 19 | Ice Shards |
| 20 | Lava Shards |
| 21 | Demon Keys|
| 22 | Total Orbs Collected |
| 23 | Earth Shards |
| 24 | Blood Shards |
| 25 | Metal Shards |
| 26 | Light Shards |
| 27 | Soul Shards |
| 28 | Moons |
| 29 | Diamond Shards |
| 30 | Fire Path Progress |
| 31 | Ice Path Progress |
| 32 | Poison Path Progress |
| 33 | Shadow Path Progress |
| 34 | Lava Path Progress |
| 35 | Earth Path Progress |
| 36 | Blood Path Progress |
| 37 | Metal Path Progress |
| 38 | Light Path Progress |
| 39 | Soul Path Progress |
| 40 | Amount of Completed Gauntlets |
| 41 | List Rewards |
| 42 | Completed Insane Levels |
| 43 | Event Keys |
| unique_{LevelID}_{Coins Collected} | The Coins Collected on the Official Levels
| unique_secretB03 | Glubfub coin |
| unique_secret04 | Official level page secret coin |
| unique_secret06 | 'Sparky' coin |


#### GS_completed
all completed levels

| Key | Value |
|-----|-------|
| `n_{levelID}`| Completed official level in normal mode |
| `c_{levelID}` | Completed online level |
| `pack_{packID}` | Completed Map Pack |
| `d_{levelID}` | Completed daily |
| `g_{levelID}` | Completed gauntlet |
| `star_{levelID}` | Collected Stars |
| `dstar_{timelyID}` | Collected Stars for daily |
| `gstar_{levelID}` | Collected Stars for gauntlet |
| `demon_{levelID}` | Collected Demon |
| `ddemon_{timelyID}` | Collected Demon for weekly|
| `gdemon_{levelID}` | Collected Demon for gauntlet|
| `e_{levelID}` | Completed event |

### GS_3

GS_3 contains info about all levels you have collected unverified coins for

| Structure |
|-----------|
|`{levelID}_{coins collected}`|

### GS_4

GS_4 contains info about all levels you have collected verified coins for

| Structure |
|-----------|
|`{levelID}_{coins collected}`|

### GS_5

GS_5 contains info about the amount of stars each map pack rewards

| Structure |
|-----------|
|`<k>pack_{packID}</k><s>{stars Rewarded}</s>`|

### GS_6

GS_6 contains all the Purchased Icons on your account

| Structure |
|-----------|
|`<k>{ListingID}</k><s>{Price}</s>`|

### GS_7

GS_7 is the Highscore attempt of levels

| Structure |
|-----------|
|`<k>{levelID}</k><s>{percentage}</s>`|

### GS_8

Unused

### GS_9

GS_9 provides info about how many stars downloaded levels give to you

| Structure |
|-----------|
|`<k>{levelID}</k><s>{rewardedStars}</s>`|

### GS_10

GS_10 Contains the high score of All Official Levels you have played

| Structure |
|-----------|
|`<k>{ID}</k><s>{percentage}</s>`|

### GS_11

GS_11 Contains the reward data for the Timely Chests you have opened

| key | Description |
|-----|-------|
| `<k>{ChestType}_{ChestID}</k>` | reward Object |
| `k_{number}` | RewardItems -> the number corresponds to how many RewardItems the player will recieve |
| `kCEK` | The [encoder keys](/resources/client/gamesave/kCEK#kcek-8-and-9-structure) |

#### Chest Types

| ID | type |
|----|------|
| 1 | 4 hour chest |
| 2 | 24 hour chest |

### GS_12

GS_12 Contains the currently active [Quests](/resources/client/gamesave/quests)

### GS_14

The Diamond Rewards Collected from Challenges and Daily levels

| structure | type |Description |
|-----------|-------|------------|
|`{TypeID}{challenge/dailyID}` | int | The Number of Diamonds Earned | 

#### TypeID's

| Types | Description |
|-------|-------------|
| c | Challenges/quests |
| d | daily levels |

### GS_15

GS_15 Contains the upcoming [Quests](/resources/client/gamesave/quests)

### GS_16

GS_16 contains the High Scores for daily and weekly demons

| Structure |
|-----------|
|`<k>{TimelyID}</k><s>{percentage}</s>`|

### GS_17

GS_17 contains the Stars rewarded for daily and weekly demons

| Structure |
|-----------|
|`<k>{TimelyID}</k><s>{Stars}</s>`|

### GS_18

GS_18 contains the High Scores for Gauntlet Levels

| Structure |
|-----------|
|`<k>{levelID}</k><s>{percentage}</s>`|

### GS_19

GS_19 Contains the reward data for the Treasure room chests you have opened

| key | Description |
|-----|-------|
| `<k>{ChestID}</k>` | reward Object |
| `k_{number}` | RewardItems -> the number corresponds to how many RewardItems the player will recieve |
| `kCEK` | The [encoder keys](/resources/client/gamesave/kCEK#kcek-8-and-9-structure) |

### GS_20

GS_20 seems to be the total amount of Demon Keys you have had over your accounts lifespan

### GS_21

GS_21 Contains the reward data for the gauntlets you have completed

| key | Description |
|-----|-------|
| `g_{GauntletID}` | the gauntlet number |
| `00{ChestID}` | ChestIDs for Reward Chests |
| `k_{number}` | RewardItems -> the number corresponds to how many RewardItems the player will recieve |
| `kCEK` | The [encoder keys](/resources/client/gamesave/kCEK#kcek-8-and-9-structure) |

#### Chest IDs

| ID | how to get |
|-----|-------|
| 0001 | Beat the Challenge |
| 0002 | Release the Demon Guardian |
| 0003 | Chamber of Time Chest|
| 0004 | Treasure Room 50 key Chest|
| 0005 | Treasure Room 100 Key Chest|
| 0006 | Treasure Room 200 Key Chest|
| 0007 | YouTube Chest |
| 0008 | Twitter Chest |
| 0009 | Facebook Chest |
| 0010 | Twitch Chest |
| 0011 | Discord Chest |
| 0022 | Zolguroth Chest (repeatedly poke Shopkeeper and return to shop) |
| 0023 | Help Button Chest |
| 0024 | Reddit Chest |

All chests between 0012 and 0021 are Ad Chests from the free versions of the game.


### GS_22

GS_22 Contains reward data for all of the ad chests from Geometry Dash world

| Key | Value |
|-----|-------|
| `{unix Timestamp}` | orbs |

### GS_23

GS_23 contains the High Scores for Gauntlet Levels

| Structure |
|-----------|
|`<k>{levelID}</k><s>{percentage}</s>`|

### GS_24

GS_24 contains the High Scores for daily and weekly demons

| Structure |
|-----------|
|`<k>{TimelyID}</k><s>{percentage}</s>`|

### GS_25

GS_25 Contains the reward data for every weekly demon you have completed

| key | Description |
|-----|-------|
| `d100{number}` | The Timely ID of the weekly Demon you have beaten |
| `k_{number}` | RewardItems -> the number corresponds to how many RewardItems the player will recieve |
| `kCEK` | The [encoder keys](/resources/client/gamesave/kCEK#kcek-8-and-9-structure) |

### GS_26

GS_26 is your currently active path. The value is the same as the path key in GS_value. Contary to most GS's, it's an integer value and not a dictionary.

| Structure |
|-----------|
| `<k>GS_26</k><i>{path}</i>` |

### GS_27

GS_27 contains the List Rewards you have acquired

| Structure |
|-----------|
|`<k>lr_{listID}</k><s>{diamonds}</s>`|

### GS_28

GS_28 contains your enabled items. All of them are strings (although they act as booleans). The animations (18-20) are set to 1 after doing.. something? even if not unlocked. The music unlocker is set to 1 by default as soon as you start the game. The music customize is set to 1 permanently when you unlock it.

| Key   | Description          |
|-------|----------------------|
| 17_12 | Music Unlocker       |
| 18_12 | Slow Robot Walk      |
| 19_12 | Fast Robot Run       |
| 20_12 | Spider Naruto Run    |
| 21_12 | Music Customizer     |

### GS_29

GS_29 is unknown but is related to trying to fix some path bug. Contary to most GS's, it's a boolean value and not a dictionary.

| Structure |
|-----------|
| `<k>GS_29</k><t />` |

### GS_30

GS_30 contains the Secret Room (Wraith) chests you have opened

| Structure                          |
| ----------------------------------- |

### GS_31

GS_31 contains the info about event level reward chests

| Key               | Description                                                                        |
| ------------------ | ------------------------------------------------------------------------------------- |
| `d{eventID}`       | Event Level Chest                                                                     |
| `k_{number}`       | RewardItems -> the number corresponds to how many RewardItems the player will recieve |
| `kCEK`             | The [encoder keys](/resources/client/gamesave/kCEK#kcek-8-and-9-structure)            |

### GS_32

GS_32 will contain the user's unlocked gauntlets, currently unused

## 1.5. GJA
| Key | Value |
|---|---|
| GJA_001 | Username |
| GJA_002 | Plaintext password (2.1 and below) |
| GJA_003 | AccountID |
| GJA_004 | Session ID (unused) |
| GJA_005 | Your password with GJP2 Encryption |
## 1.6. LLM
| Key | Value |
|---|---|
| LLM_01 | Local Levels |
| LLM_02 | Hardcoded to binaryVersion |
| LLM_03 | Local Lists |
## 1.7. MDLM
| Key | Value |
|---|---|
| MDLM_001 | Dictionary of SongInfoObject |
| MDLM_002 | Song Priority of a song |
| MDLM_003 | Unknown (dict) |
## 1.8. KBM
| Key | Value |
|---|---|
| KBM_001 | Keybindings |
| KBM_002 | Keybindings |

## 1.9. Gamevalues (gv)
Game Variables (gvs) are found inside of the [ValueKeeper](/resources/client/gamesave/valueKeeper.md) Dictionary and each gv value corresponds with a specific ingame setting

### GV Structure

| Key | Value |
| ------- | ----------------------------------------------------------- |
| gv_0001 | Follow PLayer (editor) |
| gv_0002 | play Music (editor) |
| gv_0003 | Swipe (editor) |
| gv_0004 | Free Move (editor) |
| gv_0005 | Delete filter (editor) |
| gv_0006 | Delete custom objectID (editor) |
| gv_0007 | Toggle rotate (editor) |
| gv_0008 | Toggle snap (editor) |
| gv_0009 | Ignore Damage (editor) |
| gv_0010 | flip2PlayerControls |
| gv_0011 | alwaysLimitControls |
| gv_0012 | showedCommentingRules |
| gv_0013 | increaseMaxUndoRedo |
| gv_0014 | disableExplosionShake |
| gv_0015 | flipPauseButton |
| gv_0016 | showedSongTOS |
| gv_0018 | noSongLimit |
| gv_0019 | songsInMemory |
| gv_0022 | higherAudioQuality |
| gv_0023 | smoothFix |
| gv_0024 | showCursor In-Game |
| gv_0025 | Windowed Mode |
| gv_0026 | autoRetry |
| gv_0027 | autoCheckpoints |
| gv_0028 | disableThumbstick |
| gv_0029 | showedOptionsText |
| gv_0030 | vsyncEnabled |
| gv_0031 | callGLFinish -> seems to be unused |
| gv_0032 | forceTimerEnabled -> seems to be unused |
| gv_0033 | ChangeSongPath |
| gv_0034 | GameCenterEnabled |
| gv_0035 | Small Grid Step - 1.9 |
| gv_0036 | PreviewMode (editor) |
| gv_0037 | showGround (editor) |
| gv_0038 | showGrid (editor) |
| gv_0039 | gridOnTop (editor) |
| gv_0040 | showPercentage |
| gv_0041 | showObjectInfo (editor) |
| gv_0042 | increaseMaxLevels |
| gv_0043 | effectLinesEnabled (editor) |
| gv_0044 | drawTriggerBoxes (editor) |
| gv_0045 | Show Hitboxes (editor) |
| gv_0046 | hideUIOnTest (editor) |
| gv_0047 | showedProfileText |
| gv_0048 | viewedOwnProfile |
| gv_0049 | buttonsPerRow (editor) |
| gv_0050 | buttonRows (editor) |
| gv_0051 | showedNGMessage (editor) |
| gv_0052 | Enable Faster Reset |
| gv_0053 | Free Games Popup |
| gv_0055 | checkIfServerOnline.php |
| gv_0056 | disableObjectAlert |
| gv_0057 | editorHoldToSwipe |
| gv_0058 | durationLines (editor) |
| gv_0059 | swipeCycleMode |
| gv_0060 | defaultMiniIcon |
| gv_0061 | switchSpiderTeleportColor |
| gv_0062 | switchDashFireColor |
| gv_0063 | showedUnverifiedCoinsMessage |
| gv_0064 | selectFilter (editor) |
| gv_0065 | enableMoveOptimization |
| gv_0066 | highCapacityMode |
| gv_0067 | highStartPosAccuracy |
| gv_0068 | quickCheckpointMode |
| gv_0069 | commentMode |
| gv_0070 | showedUnlistedLevelMessage |
| gv_0071 | Hide Practice Buttons |
| gv_0072 | disableGravityEffect |
| gv_0073 | newCompletedFilter |
| gv_0074 | showRestartButton |
| gv_0075 | Disable Comments |
| gv_0076 | Disable Posts |
| gv_0077 | Only Curated Levels |
| gv_0078 | hideBackground (editor) |
| gv_0079 | hideGridOnPlay (editor) |
| gv_0081 | disableShakeEffects |
| gv_0082 | disableHighObjectAlert |
| gv_0083 | disableSongAlert |
| gv_0084 | manualOrder |
| gv_0088 | smallCommentsMode |
| gv_0089 | extendedInfoMode |
| gv_0090 | autoloadComments |
| gv_0091 | myLevels folder number |
| gv_0092 | onlineLevels folder number |
| gv_0093 | increaseLocalLevelsPerPage |
| gv_0094 | moreCommentsMode |
| gv_0095 | flippyUselessMode ("Do Not..." (formerly "Just Don't")) |
| gv_0096 | switchWaveTrailColor |
| gv_0097 | enableLinkControls (editor) |
| gv_0098 | levelLeaderboardType |
| gv_0099 | showLeaderboardPercent |
| gv_0100 | practiceDeathEffect |
| gv_0101 | forceSmoothFix |
| gv_0102 | smoothFixInEditor |
| gv_0103 | Layer Locking (editor) |
| gv_0104 | Record Order |
| gv_0105 | StartPos Playback |
| gv_0106 | Show Meltdown Promo |
| gv_0108 | Auto Low Detail |
| gv_0109 | Level Info Label |
| gv_0110 | Fast Editor Preview |
| gv_0112 | Increase Scale Limit (editor) |
| gv_0113 | Flip Platformer Controls |
| gv_0115 | Show FPS |
| gv_0116 | Use Custom FPS |
| gv_0117 | Preview Particle |
| gv_0118 | Preview Animation |
| gv_0119 | Dont Save Level Data |
| gv_0121 | Hide Invisible (editor) |
| gv_0122 | Disable Menu Music |
| gv_0125 | Unlock Practice Music (editor) |
| gv_0126 | Decimal Percentage |
| gv_0127 | Save Gauntlet Levels |
| gv_0128 | Lock Cursor In-Game |
| gv_0129 | Disable Portal Labels |
| gv_0130 | Enable Orb Labels |
| gv_0131 | Use Nearby as Reference (smart template) |
| gv_0132 | Dont Delete (smart template) |
| gv_0133 | Group ID Filter (editor) |
| gv_0134 | Hide Attempts |
| gv_0135 | Hide Attempts in Practice Mode |
| gv_0136 | Enable Extra LDM |
| gv_0137 | Hide Particle Icons (editor) |
| gv_0139 | Color Channel Filter (editor) |
| gv_0140 | Disable Orb Scale |
| gv_0141 | Disable Trigger Orb Scale |
| gv_0142 | Reduce Audio Quality |
| gv_0144 | Audio Visualizer |
| gv_0145 | Show Time |
| gv_0146 | Disable Checkpoints |
| gv_0147 | Search Folder Name (SFX Library) |
| gv_0148 | Compact Mode (SFX Library) |
| gv_0149 | Show Clicks (editor) |
| gv_0150 | Auto Pause on Test (editor) |
| gv_0151 | Start Optimization (editor) |
| gv_0152 | Hide Path (editor) |
| gv_0153 | Explode Player on Death |
| gv_0155 | Disable Shader Anti-Aliasing |
| gv_0156 | Disable Paste State Groups (editor) |
| gv_0157 | Level Upload Guidelines Shown |
| gv_0158 | Preview Shaders (editor) |
| gv_0159 | Audio Fix 01 |
| gv_0163 | Enable Quick Keys |
| gv_0164 | Level Leaderboard Mode |
| gv_0166 | Show Hitboxes (in practice mode) |
| gv_0167 | Confirm Exit |
| gv_0168 | Fast Menu |
| gv_0169 | Small Warp Buttons (editor) |
| gv_0170 | Borderless Fullscreen |
| gv_0171 | Disable Player Hitbox |
| gv_0172 | Disable Shake (2.2) |
| gv_0173 | Ignore Damage (playtesting outside of editor) |
| gv_0174 | Hide Playtest Text |
| gv_0175 | Borderless Fix |
| gv_0176 | Click On Steps |
| gv_0177 | Click Between Steps |
| gv_0178 | Perf Log (editor, in particle emitter object) |
| gv_0179 | Hitbox on Death |
| gv_0180 | Quick Disable Link Controls (editor, the gray link button) |
| gv_0181 | Static Trace Arrows (editor) |
| gv_0191 | Legacy Move Menu |
| gv_0196 | Disable Player Trail |
| gv_0197 | Disable Wave Trail |
| gv_0200 | Local Markers |
| gv_0201 | Disable Anim (death markers) |
| gv_0202 | Global Markers |
| gv_0203 | Disable Auto Creator Controls |
| gv_0204 | Switch Color (death markers) |
| gv_0205 | Death Marker Frequency Slider |
| gv_0206 | Disable Sound (death markers) |
| gv_0207 | Disable Markers |
| gv_0208 | Hide Labels (death markers) |
| gv_0210 | Limited Labels (death markers) |
| gv_0211 | Death Marker Duration Slider |
| gv_0212 | Skip Death Triggers |
| gv_0213 | Show Hitbox Trail |
| gv_0214 | Hitbox Trail On Death |
| gv_0215 | Hitbox Trail (editor) |
| gv_0216 | Live Practice Markers |
| gv_0217 | Hitbox Trail Highlight |
| gv_0218 | Safe Mode |
| gv_0219 | Offline Mode |
| gv_0228 | Parental Controls Lock |
| gv_0234 | No Auto-Checkpoints For Start Pos |
| gv_0235 | Hide Mod Buttons |
| gv_0239 | Notifications Bitmask (which notification types are disabled) |
| gv_0242 | Disable Anchor Snap |
| gv_0243 | Disable Messaging |
| gv_0245 | Disable Pinch |
| gv_0246 | Agreed to Messaging Rules |

### Unlocked Game Variables

Unlocked Game Variables (ugv) are used to check if ingame events have been completed

| Key | Value|
| --- |------------|
| ugv_1 | Unlocked the challenge |
| ugv_2 | Gubflub hint |
| ugv_3 | Gubflub hint 2 |
| ugv_4 | The Challenge completed |
| ugv_5 | Treasure Room unlocked |
| ugv_6 | Chamber of Time unlocked |
| ugv_7 | Chamber of Time Discovered |
| ugv_8 | shown Master Emblem |
| ugv_9 | GateKeeper Dialog |
| ugv_10 | Scratch Dialog |
| ugv_11 | Secret Shop unlocked |
| ugv_12 | Spoken to Demon Guardian |
| ugv_13 | Demon Freed |
| ugv_14 | Has Demon Key 1 |
| ugv_15 | Has Demon Key 2 |
| ugv_16 | Has Demon Key 3 |
| ugv_17 | Shop Keeper first dialog (500 orbs) |
| ugv_18 | online levels unlocked in GDW |
| ugv_19 | Demon Discovered in Demon Room |
| ugv_20 | Community shop unlocked |
| ugv_21 | PotBor dialog |
| ugv_22 | YouTube Chest unlocked |
| ugv_23 | FaceBook Chest unlocked |
| ugv_24 | Twitter Chest unlocked |
| ugv_25 | Explorers unlocked |
| ugv_26 | Twitch Chest unlocked |
| ugv_27 | Discord Chest unlocked |
| ugv_28 | Clicked The Tower |
| ugv_29 | Entered The Tower |
| ugv_30 | Accepted Geometry Dash's Terms of Service |
| ugv_31 | Zolguroth Encountered |
| ugv_32 | Reddit Chest unlocked |
| ugv_33 | The Tower Floor 1 completed |
| ugv_34 | Diamond Shop Unlocked |
| ugv_35 | Mechanic Unlocked |
| ugv_36 | Mechanic Dialogue |
| ugv_37 | Diamond Shopkeeper Dialogue |

### Undiscovered/unknown GV's

| GV_ID | Existance Proven | notes |
|-------|--------|-------|
| 0017 | **False** | |
| 0020 | **False** | |
| 0021 | **False** | |
| 0054 | **False** ||
| 0080 | **False** ||
| 0085 | **False** ||
| 0086 | **False** ||
| 0087 | **False** ||
| 0114 | **True** | Used in `PlayerObject` (**2019 Leak**) |
| 0120 | **True** | Shown in RobTop's 2020 Twitch Streams |

## 1.10 kCEK used in this file
### kCEK 8 and 9 Structure

`GJRewardItem` is a dictionary which contains `GJRewardObject` 

| key | encoder Key | use/description |
|-----|-------------|-------------|
| 1 | 8 | Depends on the [GS_Value](/resources/client/gamesave/GS_Value) it is inside of |
| 2 | 8 | Depends on the [GS_Value](/resources/client/gamesave/GS_Value) it is inside of |
| 3 | 8 | GJRewardObject |
| 1 | 9 | item type |
| 2 | 9 | customItemID |
| 3 | 9 | reward amount |
| 4 | 9 | itemUnlockValue -> [presumedly contains data that unlocks an item in the garage?](https://media.discordapp.net/attachments/801840133355470888/806137688905875496/unknown.png) |

#### ItemIDs

| key | value |
|-----|-------|
| 1 | Fire Shard |
| 2 | Ice Shard |
| 3 | Poison Shard |
| 4 | Shadow Shard |
| 5 | Lava Shard |
| 6 | Demon Key |
| 7 | Orbs |
| 8 | Diamonds |
| 9 | Custom Object (shares the same ID for Demon guardian key, icons and colours) |
| 10| Earth Shard |
| 11| Blood Shard |
| 12| Metal Shard |
| 13| Light Shard |
| 14| Soul Shard |
| 15| Gold Key |

#### GJRewardItem GS Keys

##### GS_11

| key | value |
|-----|-------------|
| 1 | ChestID | 
| 2 | ChestType | 

###### Chest Types

| ID | type |
|----|------|
| 1 | 4 hour chest |
| 2 | 24 hour chest |