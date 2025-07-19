use crate::models::{images::SysPrompt, PackModel, TrainingModelModel};
use derive_more::Display;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::EnumString;
use tera::{Context, Tera};
use thiserror::Error;

pub const MODEL_UUID: &str = "eec306f5-70ba-4ff3-a888-6e6843d86caf";
const _TENSOR_PATH: &str =
    "https://v3.fal.media/files/penguin/jbAfBP9Q0cxq4bxK9hqjM_pytorch_lora_weights.safetensors";

#[derive(Debug, Error)]
pub enum PromptRenderError {
    #[error("Failed to render prompt: {0}")]
    RenderError(#[from] tera::Error),
}

const _DEFAULT_PROMPT_TEMPLATE: &str = "{{trigger}} {{user}}. {{trigger}} is a {{ethnicity}} {{sex}} with {{eyes}} eyes, aged {{age}}.";

const _PROMPT_V2: &str = "{{trigger}}. {{trigger}} is a {{ethnicity}} {{sex}}{% if bald %} who is bald{% endif %} with {{eyes}} eyes, aged {{age}}. {{based_on}}.
Valentine’s Day fantasy portrait of a stunning {{sex}} in elegant, heart-inspired attire, softly glowing in warm pink and red tones.  
She poses gracefully with natural limb proportions — both arms and legs fully visible and correctly formed — while surrounded by romantic elements like rose petals, silky fabrics, and delicate lighting.  
The setting is playful yet classy, evoking charm, allure, and a modern fairytale vibe.";

const _PROMPT_TEXT: &str = "Valentine's Day fantasy portrait of a stunning woman in elegant, heart-inspired attire, softly glowing in warm pink and red tones. 
She poses gracefully with natural limb proportions both arms and legs fully visible and correctly formed—while surrounded by romantic elements like rose petals, silky fabrics, and delicate lighting. 
The setting is playful yet classy, evoking charm, allure, and a modern fairytale vibe.";

const PROMPT_THEMELESS_TEMPLATE: &str = "{{trigger}}. {{trigger}} is a {{ethnicity}} {{sex}}{% if bald %} who is bald{% endif %} with {{eyes}} eyes, aged {{age}}. {{based_on}}.
 A fantasy portrait of a stunning {{sex}} in a {{theme}} style. The subject is wearing {{attire}}. The scene is set in {{background_setting}}, featuring elements like {{background_elements}}.
  The lighting is {{lighting_style}}, creating {{lighting_effect}}. The color palette is dominated by {{color_palette}}. 
  Critically, ensure natural limb proportions, with all arms and legs fully visible and correctly formed.{% if extra_info %} {{extra_info}}{% endif %}";

// Training Model
const TRIGGER: &str = "trigger";
const AGE: &str = "age";
const SEX: &str = "sex";
const EYES: &str = "eyes";
const ETHNICITY: &str = "ethnicity";
const BALD: &str = "bald";
const BASED_ON: &str = "based_on";

// Themes
const THEME: &str = "theme";
const ATTIRE: &str = "attire";
const BACKGROUND_SETTING: &str = "background_setting";
const BACKGROUND_ELEMENTS: &str = "background_elements";
const LIGHTING_STYLE: &str = "lighting_style";
const LIGHTING_EFFECT: &str = "lighting_effect";
const COLOR_PALETTE: &str = "color_palette";
const EXTRA_INFO: &str = "extra_info";

pub fn formatted_prompt(
    pack: &PackModel,
    themes_variables: Option<Themes>,
    training_model: Option<TrainingModelModel>,
) -> Result<SysPrompt, PromptRenderError> {
    let training_model = match training_model {
        Some(model) => model,
        None => return Ok(SysPrompt::new(&pack.pack_prompts)),
    };
    let themes_variables = match themes_variables {
        Some(model) => model,
        None => return Ok(SysPrompt::new(&pack.pack_prompts)),
    };

    let mut ctx = Context::new();
    ctx.insert(TRIGGER, &training_model.trigger_word);
    ctx.insert(AGE, &training_model.age);
    ctx.insert(SEX, &training_model.sex);
    ctx.insert(EYES, &training_model.eye_color.to_string());
    ctx.insert(ETHNICITY, &training_model.ethnicity.to_string());
    ctx.insert(BALD, &training_model.bald);
    ctx.insert(BASED_ON, &training_model.based_on.to_string());
    ctx.insert(THEME, themes_variables.theme);
    ctx.insert(ATTIRE, themes_variables.attire);
    ctx.insert(BACKGROUND_SETTING, themes_variables.background_setting);
    ctx.insert(BACKGROUND_ELEMENTS, themes_variables.background_elements);
    ctx.insert(LIGHTING_STYLE, themes_variables.lighting_style);
    ctx.insert(LIGHTING_EFFECT, themes_variables.lighting_effect);
    ctx.insert(COLOR_PALETTE, themes_variables.color_palette);
    ctx.insert(EXTRA_INFO, themes_variables.extra_info);

    let rendered = Tera::one_off(PROMPT_THEMELESS_TEMPLATE, &ctx, false)?;
    Ok(SysPrompt::new(rendered))
}

#[derive(Clone, Debug, Serialize, Deserialize, EnumString, PartialEq, Eq, Display)]
pub enum Theme {
    #[strum(to_string = "sexy-valentine-pack")]
    Valentine,
    #[strum(to_string = "sexy-cyberpunk-pack")]
    SexyCyberpunk,
    #[strum(to_string = "sexy-halloween-pack")]
    SexyHalloween,
    #[strum(to_string = "sexy-easter-pack")]
    SexyEaster,
    #[strum(to_string = "headshots-pack")]
    Headshots,
    #[strum(to_string = "corporate-headshots-pack")]
    CorporateHeadshots,
    #[strum(to_string = "model-headshots-pack")]
    ModelHeadshots,
    #[strum(to_string = "mob-wife-pack")]
    MobWife,
    #[strum(to_string = "mobster-pack")]
    Mobster,
    #[strum(to_string = "cosplay-pack")]
    Cosplay,
    #[strum(to_string = "nature-pack")]
    Nature,
    #[strum(to_string = "spiritual-pack")]
    Spiritual,
    #[strum(to_string = "villain-pack")]
    VillainVibes,
    #[strum(to_string = "elf-queen-fantasy-pack")]
    ElfQueen,
    #[strum(to_string = "luxury-travel-pack")]
    LuxuryTravel,
    #[strum(to_string = "gamer-girl-pack")]
    GamerGirl,
    #[strum(to_string = "tinder-pack")]
    Tinder,
    #[strum(to_string = "bumble-pack")]
    Bumble,
    #[strum(to_string = "hinge-pack")]
    Hinge,
    #[strum(to_string = "okcupid-pack")]
    OkCupid,
    #[strum(to_string = "fitness-model-pack")]
    FitnessModel,
    #[strum(to_string = "sexy-summer-beach-pack")]
    SexySummerBeach,
    #[strum(to_string = "cottagecore-pack")]
    Cottagecore,
    #[strum(to_string = "model-walkway-pack")]
    ModelWalkway,
    #[strum(to_string = "doctor-headshots")]
    DoctorHeadshots,
    #[strum(to_string = "nurse-headshots")]
    NurseHeadshots,
    #[strum(to_string = "sexy-nurse")]
    SexyNurse,
    #[strum(to_string = "wedding")]
    Wedding,
    #[strum(to_string = "flight-attendant")]
    FlightAttendant,
    #[strum(to_string = "selfie")]
    AISelfie,
    #[strum(to_string = "firefighter-action")]
    FirefighterAction,
    #[strum(to_string = "cyberpunk-mercenary")]
    CyberpunkMercenary,
    #[strum(to_string = "gourmet-chef")]
    GourmetChef,
    #[strum(to_string = "wasteland-survivor")]
    WastelandSurvivor,
    #[strum(to_string = "follow-me-to")]
    FollowMeTo,
    #[strum(to_string = "world-traveler")]
    WorldTraveler,
    #[strum(to_string = "royal-knight")]
    RoyalKnight,
    #[strum(to_string = "house-party")]
    HouseParty,
    #[strum(to_string = "pilot-headshot")]
    PilotHeadshot,
    #[strum(to_string = "viking-warrior")]
    VikingWarrior,
    #[strum(to_string = "shieldmaiden")]
    Shieldmaiden,
    #[strum(to_string = "pirate-captain")]
    PirateCaptain,
    #[strum(to_string = "sexy-librarian")]
    SexyLibrarian,
    #[strum(to_string = "lifeguard-duty")]
    LifeguardDuty,
    #[strum(to_string = "college-party")]
    CollegeHouseParty,
    #[strum(to_string = "tattooed-style")]
    TattooedStyle,
    #[strum(to_string = "wizard-sorceress")]
    WizardSorceress,
    #[strum(to_string = "red-carpet-gala")]
    RedCarpetGala,
    #[strum(to_string = "steampunk-inventor")]
    SteampunkInventor,
    #[strum(to_string = "holi")]
    Holi,
    #[strum(to_string = "reporter-live")]
    ReporterLive,
    #[strum(to_string = "goth-style")]
    GothStyle,
}

pub struct Themes {
    pub pid: &'static str,
    pub theme: &'static str,
    pub attire: &'static str,
    pub background_setting: &'static str,
    pub background_elements: &'static str,
    pub lighting_style: &'static str,
    pub lighting_effect: &'static str,
    pub color_palette: &'static str,
    pub extra_info: &'static str,
}

impl Themes {
    pub fn from_title_url(title_url: &str) -> Option<Self> {
        Theme::from_str(title_url).ok().map(Self::new)
    }
    pub fn new(theme: Theme) -> Self {
        match theme {
            Theme::TattooedStyle => Self {
                pid: "e2e55fc0-4346-4432-a41c-7b1048e2fe53",
                theme: "a tattooed style portrait",
                attire: "minimal clothing or a leather jacket to showcase full sleeve, chest, or neck tattoos",
                background_setting: "an urban or studio background",
                background_elements: "elements that complement a gritty or clean style",
                lighting_style: "moody and dim lighting",
                lighting_effect: "emphasizing the tattoo art and personal style",
                color_palette: "high-contrast, dark, and urban tones",
                extra_info: "The subject has a confident expression and a strong, bold pose. High detail is required.",
            },

            Theme::WizardSorceress => Self {
                pid: "3dcae293-1e4f-4c7d-97ad-a689d78828bc",
                theme: "a realistic wizard or sorceress portrait",
                attire: "flowing robes",
                background_setting: "a forest",
                background_elements: "trees, foliage, and magical spell effects",
                lighting_style: "natural lighting, illuminated by a spell effect",
                lighting_effect: "creating a magical and realistic atmosphere",
                color_palette: "earthy forest tones contrasted with bright magical energy",
                extra_info: "The subject is casting a fireball, ice, or electricity spell.",
            },

            Theme::RedCarpetGala => Self {
                pid: "3e91a578-2d36-489a-af4c-75fd16b65ebf",
                theme: "a red carpet gala event",
                attire: "elegant evening attire such as a gown or tuxedo, with polished accessories",
                background_setting: "a red carpet with a glamorous backdrop",
                background_elements: "event banners, camera flashes, or elegant decor",
                lighting_style: "bright event lighting",
                lighting_effect: "creating a radiant and glamorous look",
                color_palette: "rich, formal colors like black, red, gold, and silver",
                extra_info: "The subject has a confident pose and a radiant expression. The style is realistic with detailed clothing.",
            },

            Theme::SteampunkInventor => Self {
                pid: "083dddda-b4f0-4bf5-869c-6e9cbe1ed417",
                theme: "a steampunk inventor portrait",
                attire: "steampunk fashion including brass goggles, leather jackets, and mechanical accessories",
                background_setting: "an industrial background",
                background_elements: "gears, pipes, and steam",
                lighting_style: "warm, industrial lighting",
                lighting_effect: "creating a creative and detailed atmosphere",
                color_palette: "brass, copper, bronze, and dark leather tones",
                extra_info: "The subject has an expressive pose showing confidence and creativity. The style is realistic.",
            },

            Theme::Holi => Self {
                pid: "f84af94b-d6fa-46ee-8630-c2e424439654",
                theme: "a Holi festival celebration",
                attire: "simple clothing, often white, to contrast the colored powder",
                background_setting: "a festive outdoor celebration",
                background_elements: "other people and clouds of colorful powder",
                lighting_style: "bright, natural lighting",
                lighting_effect: "highlighting the vibrant colors and joyful atmosphere",
                color_palette: "an explosion of vibrant colors like pink, blue, yellow, and green",
                extra_info: "The subject is covered in vibrant Holi powder and is smiling joyfully, in a dynamic pose. The style is realistic with detailed textures and colorful powder effects.",
            },

            Theme::ReporterLive => Self {
                pid: "f840eea9-814c-4968-a76f-281e69ba4754",
                theme: "a live news reporter",
                attire: "professional attire, holding a microphone",
                background_setting: "an urban or event background",
                background_elements: "elements of a live news scene",
                lighting_style: "natural lighting",
                lighting_effect: "creating a realistic and professional look",
                color_palette: "natural, real-world colors",
                extra_info: "The subject is reporting live on camera with an expressive, focused, and confident expression. Realistic style with detailed surroundings.",
            },

            Theme::GothStyle => Self {
                pid: "67307ec8-3b8b-4e8d-9cba-4986f506255c",
                theme: "a realistic gothic fashion portrait",
                attire: "natural layered velvet, lace, and leather clothing with refined bold makeup and subtle gothic accessories",
                background_setting: "a shadowed urban nightscape or worn gothic architecture",
                background_elements: "architectural details or city elements at night",
                lighting_style: "soft, moody lighting with natural shadows",
                lighting_effect: "creating a calm or introspective atmosphere",
                color_palette: "deep blacks, rich velvets, and dark, moody tones",
                extra_info: "The subject has a calm or introspective expression with realistic skin textures.",
            },
            Theme::Wedding => Self {
                pid: "6ba507a8-87e3-4a10-b697-b8677ab17b96",
                theme: "a beautiful bride portrait",
                attire: "a flowing white wedding dress with lace details, elegant jewelry, and a bouquet",
                background_setting: "a romantic outdoor garden or elegant indoor setting",
                background_elements: "elements that complement a wedding scene",
                lighting_style: "soft, natural, or studio lighting",
                lighting_effect: "creating a romantic and elegant atmosphere",
                color_palette: "whites, creams, and soft floral colors",
                extra_info: "The subject has a soft smile with hair styled in loose waves or an elegant updo. Studio quality photography.",
            },

            Theme::FlightAttendant => Self {
                pid: "9abbc9e9-050f-436d-91d1-b3e80083e72b",
                theme: "a professional flight attendant portrait",
                attire: "a stylish airline uniform with accessories such as a scarf, hat, or name tag",
                background_setting: "a modern airport lounge or an aircraft interior",
                background_elements: "elements of an airplane or airport",
                lighting_style: "soft, natural, or studio lighting",
                lighting_effect: "creating a professional and friendly look",
                color_palette: "airline brand colors or professional neutral tones",
                extra_info: "The subject has styled hair and a friendly smile or calm expression. Studio quality photography.",
            },

            Theme::AISelfie => Self {
                pid: "79f009c1-b78c-420b-bc04-16997ab16dce",
                theme: "a mirror selfie",
                attire: "user-defined casual or stylish clothing",
                background_setting: "an indoor setting like a bedroom, bathroom, or living room",
                background_elements: "a mirror and a smartphone held by the subject",
                lighting_style: "natural indoor or bathroom lighting",
                lighting_effect: "creating a realistic selfie look",
                color_palette: "natural colors of the environment",
                extra_info: "The subject is standing and taking a photo of themselves in a mirror using a smartphone.",
            },

            Theme::FirefighterAction => Self {
                pid: "d5293d28-b649-429a-8009-9fb92c73b472",
                theme: "a firefighter in an intense action scene",
                attire: "full protective gear, including a helmet and gloves, holding firefighting equipment like a hose or axe",
                background_setting: "an outdoor or indoor emergency scene",
                background_elements: "smoke and fire",
                lighting_style: "dynamic lighting with motion blur",
                lighting_effect: "creating an intense and action-packed atmosphere",
                color_palette: "fiery oranges and reds against dark, smoky tones",
                extra_info: "The subject has a determined expression and a strong stance. Studio quality action photography.",
            },

            Theme::CyberpunkMercenary => Self {
                pid: "4d12b470-3f1a-4f6f-a801-0d109ae19ae7",
                theme: "a cyberpunk mercenary portrait",
                attire: "futuristic armor and cybernetic enhancements, with futuristic tactical gear (no helmet, no mask)",
                background_setting: "a stylized, neon-lit urban background",
                background_elements: "holographic displays and futuristic city elements",
                lighting_style: "dramatic neon lighting",
                lighting_effect: "creating a cyberpunk aesthetic",
                color_palette: "vibrant neons against dark urban colors",
                extra_info: "The subject has a visible face. The style is studio quality concept art.",
            },

            Theme::GourmetChef => Self {
                pid: "5d2c90f2-fbc7-45bf-a869-e9dfcfb2e4f5",
                theme: "a gourmet chef in a culinary setting",
                attire: "a stylish chef jacket and apron",
                background_setting: "a modern professional kitchen",
                background_elements: "high-end cookware and beautifully plated gourmet dishes",
                lighting_style: "bright, natural lighting",
                lighting_effect: "creating a clean, professional atmosphere",
                color_palette: "clean whites and metallic tones with pops of color from the food",
                extra_info: "The subject has a friendly and confident expression. Studio quality professional photography.",
            },

            Theme::WastelandSurvivor => Self {
                pid: "e88330b8-fd67-448c-926b-0330d4e6babd",
                theme: "a rugged wasteland survivor",
                attire: "tattered clothes, dust-covered skin, and makeshift armor, with visible scars and survival gear",
                background_setting: "a desolate, post-apocalyptic landscape",
                background_elements: "broken machinery and dusty skies",
                lighting_style: "dramatic lighting",
                lighting_effect: "creating a gritty atmosphere",
                color_palette: "desaturated, dusty, and rusty tones",
                extra_info: "Studio quality concept art.",
            },

            Theme::FollowMeTo => Self {
                pid: "3ee88684-09ed-4a08-b1b3-75f529769ae4",
                theme: "a 'Follow Me To' style travel photo",
                attire: "flowing clothing",
                background_setting: "a scenic outdoor setting or beautiful landscape",
                background_elements: "elements of the chosen landscape",
                lighting_style: "natural lighting",
                lighting_effect: "creating a romantic and adventurous atmosphere",
                color_palette: "vibrant, natural colors of the landscape",
                extra_info: "The subject is reaching backward, holding the viewer’s hand while leading them forward, smiling over their shoulder. Cinematic travel photography style.",
            },

            Theme::WorldTraveler => Self {
                pid: "5a4b59f5-bb81-4df8-8ca1-4e5815f2f049",
                theme: "a world traveler portrait",
                attire: "casual travel outfits",
                background_setting: "famous landmarks or scenic landscapes",
                background_elements: "vibrant street life or stunning nature",
                lighting_style: "bright, natural lighting",
                lighting_effect: "creating atmospheric backgrounds that capture a sense of adventure and wanderlust",
                color_palette: "natural and location-specific colors",
                extra_info: "The subject has a relaxed and natural pose. Professional travel photography style.",
            },

            Theme::RoyalKnight => Self {
                pid: "fbb3e8ae-89a9-4b3e-995d-f8b6f367c1a9",
                theme: "a royal knight fantasy portrait",
                attire: "ornate, regal armor with intricate crests and polished metal",
                background_setting: "a medieval landscape, castle background, or misty battlefield",
                background_elements: "castles, battlefields, or medieval architecture",
                lighting_style: "dramatic lighting",
                lighting_effect: "creating an epic atmosphere",
                color_palette: "metallic silvers and golds, with rich royal colors",
                extra_info: "The subject is standing proudly with a confident and noble expression. Cinematic fantasy portrait, studio quality.",
            },

            Theme::HouseParty => Self {
                pid: "111fa4a0-a071-453b-ace5-c8951c0ff387",
                theme: "a lively house party scene",
                attire: "casual party outfits",
                background_setting: "an indoor party setting",
                background_elements: "colorful lights and happy people",
                lighting_style: "vibrant bokeh lighting",
                lighting_effect: "creating a fun and relaxed atmosphere",
                color_palette: "vibrant and colorful party lights",
                extra_info: "The subject is smiling and dancing with a fun and relaxed pose, capturing a sense of joy, friendship, and celebration. Lifestyle photography style.",
            },

            Theme::PilotHeadshot => Self {
                pid: "5ae26904-ac26-42f4-af26-e50c23e14609",
                theme: "a professional pilot headshot",
                attire: "an aviation uniform or flight suit, with optional aviation headset or sunglasses",
                background_setting: "a neutral background or a cockpit setting",
                background_elements: "cockpit instruments or a clean studio backdrop",
                lighting_style: "professional, clean, and sharp lighting",
                lighting_effect: "creating a confident and focused portrait",
                color_palette: "professional and neutral tones",
                extra_info: "Cinematic style with high detail.",
            },

            Theme::VikingWarrior => Self {
                pid: "6d6fedbb-ebe5-4fea-a2e0-85ded82829f6",
                theme: "a Viking warrior fantasy portrait",
                attire: "rugged armor, furs, and Norse-inspired gear",
                background_setting: "a Nordic landscape or battlefield background",
                background_elements: "mountains, snow, or elements of a battlefield",
                lighting_style: "dramatic lighting",
                lighting_effect: "creating a gritty, cinematic composition",
                color_palette: "earthy, cool, and metallic tones",
                extra_info: "The subject has a heroic and powerful presence. Concept art fantasy style with high detail.",
            },

            Theme::Shieldmaiden => Self {
                pid: "afee3158-54d3-47b6-a9e1-67ee5816109f",
                theme: "a shieldmaiden warrior portrait",
                attire: "rugged leather and fur armor with Norse jewelry",
                background_setting: "a Nordic wilderness or battlefield background",
                background_elements: "wild landscapes, snow, or battlefield elements",
                lighting_style: "dramatic cinematic lighting",
                lighting_effect: "creating a gritty, heroic, and powerful presence",
                color_palette: "earthy tones, leathers, and cool blues/grays",
                extra_info: "The subject has a fierce and determined expression with wind-swept hair. Highly detailed fantasy art style.",
            },

            Theme::PirateCaptain => Self {
                pid: "dccea683-f2e4-48ab-a43c-7ac9cf9958b5",
                theme: "a pirate captain fantasy portrait",
                attire: "a weathered coat and tricorn hat, with optional nautical jewelry or an eye patch",
                background_setting: "on a ship deck or with a stormy ocean background",
                background_elements: "ship rigging, waves, and stormy clouds",
                lighting_style: "dramatic lighting",
                lighting_effect: "creating a rugged, adventurous, and cinematic composition",
                color_palette: "dark, weathered tones of the sea and wood",
                extra_info: "The subject has a fearless expression with wind-swept hair. Highly detailed fantasy art style.",
            },

            Theme::SexyLibrarian => Self {
                pid: "c6c8ad37-2400-4d66-9874-d71d31e325d1",
                theme: "a sexy librarian portrait",
                attire: "a fitted blouse (slightly unbuttoned), glasses, and a form-fitting skirt",
                background_setting: "a dimly lit, cozy library setting",
                background_elements: "bookshelves",
                lighting_style: "soft, warm lighting",
                lighting_effect: "creating a cozy and alluring atmosphere",
                color_palette: "warm, indoor tones with colors from book spines",
                extra_info: "The subject has a flirty expression or a teasing glance, with a relaxed yet confident pose. Modern fashion details, cinematic composition, highly detailed.",
            },

            Theme::LifeguardDuty => Self {
                pid: "5f648c3a-6447-4bcc-bbb8-54b9ef2d0bba",
                theme: "a lifeguard on duty",
                attire: "bright swimwear, sunglasses, and rescue gear",
                background_setting: "a sunny beach or beside a lifeguard tower",
                background_elements: "the ocean, beach, and lifeguard tower",
                lighting_style: "vibrant, sunny lighting",
                lighting_effect: "creating a sun-kissed look and summer vibes",
                color_palette: "bright reds and yellows against blue ocean tones",
                extra_info: "The subject has a confident and alert expression with a relaxed and athletic pose. Highly detailed.",
            },

            Theme::CollegeHouseParty => Self {
                pid: "5f648c3a-6447-4bcc-bbb8-54b9ef2d0bba",
                theme: "a college house party scene",
                attire: "trendy streetwear or casual hoodies",
                background_setting: "an indoor house party",
                background_elements: "string lights, beer pong tables, and laughing friends",
                lighting_style: "flash photography aesthetic",
                lighting_effect: "creating a carefree and energetic vibe",
                color_palette: "vibrant, colorful party lights against a darker indoor setting",
                extra_info: "The subject has a playful or bold expression. The atmosphere is youthful and trendy. Highly detailed lifestyle scene.",
            },
            Theme::Nature => Self {
                pid: "8517ca6d-6dce-4bdb-9f71-bf4f39e5bf1d",
                theme: "a cinematic portrait deeply and interactively immersed within a rich natural setting",
                attire: "attire that harmonizes elegantly with the natural environment, like flowing fabrics in earthy tones or practical yet chic outdoor wear",
                background_setting: "a sun-dappled meadow, a serene forest, a vibrant jungle, or near a river",
                background_elements: "tall wildflowers, mossy tree bark, smooth river stones, or lush ferns",
                lighting_style: "perfect, naturalistic lighting",
                lighting_effect: "highlighting the textures of both the subject and immediate natural elements, creating a sense of tactile connection and wonder",
                color_palette: "earthy and natural tones that complement the surroundings",
                extra_info: "The subject has an elegant and natural posture, actively engaging with the environment. The composition emphasizes this interaction. Sharp focus on an expressive face (peaceful immersion, gentle curiosity). CRITICAL: Ensure all limbs are fully and correctly rendered, anatomically accurate, and appear gracefully integrated with the natural elements. High resolution and photorealistic.",
            },

            Theme::Spiritual => Self {
                pid: "34115d2d-c5c5-4df7-a234-04c5c9793c7d",
                theme: "a captivating, cinematic portrait embodying an aura of profound serenity and inner peace",
                attire: "elegant, comfortable, and peaceful attire, such as soft, flowing fabrics or simple, uncluttered designs",
                background_setting: "an abstracted, ethereal lightscape; a minimalist setting for deep meditation; or a symbolic natural environment like a still body of water or mountaintop at dawn",
                background_elements: "symbolic elements like a softly glowing orb, sand patterns, a symbolic plant, or flowing water",
                lighting_style: "artful and evocative, from soft, diffused glows to gentle, radiant beams",
                lighting_effect: "enhancing the subject's tranquil expression, deep calm, or blissful connection",
                color_palette: "gentle, natural, or luminous tones",
                extra_info: "CRUCIAL FOR VARIETY: The AI must actively vary the posture (meditative, contemplative, flowing movement) AND the specifics of the spiritual setting in each generation. Composition is a balanced medium or three-quarter view. UTMOST IMPORTANCE: Ensure all limbs are fully and correctly rendered, anatomically sound, and naturally integrated into a harmonious and graceful pose. High resolution, photorealistic.",
            },

            Theme::VillainVibes => Self {
                pid: "c1e65809-c398-4bba-9364-b2c7246c16ab",
                theme: "a fierce and commanding villain portrait",
                attire: "a style that blends elegance with danger, such as futuristic, fantasy, gothic, or high fashion; structured silhouettes, flowing fabrics, bold accessories, or armor-inspired elements",
                background_setting: "a dramatic setting such as a palace, abstract void, dystopian world, or enchanted ruins",
                background_elements: "subtle signs of power like glowing eyes, energy, smoke, or a mysterious aura",
                lighting_style: "cinematic with sharp contrast",
                lighting_effect: "creating shadows, glowing elements, or surreal effects",
                color_palette: "dark, dramatic colors suitable for a villainous theme",
                extra_info: "The subject has an intense, confident presence and moves with authority. The look can be photo-realistic or stylized with a strong mood. Full body or close-up.",
            },

            Theme::ElfQueen => Self {
                pid: "6c6d5904-452d-4af8-9dc1-13d3ab4b2646",
                theme: "a majestic elf queen or warrior high-fantasy portrait",
                attire: "regal robes, enchanted armor, or nature-infused gear, potentially holding a glowing staff, bow, or sword",
                background_setting: "a magical forest, ancient temple, elven throne room, or mountain overlook",
                background_elements: "mystical ambiance, mist, and natural light rays",
                lighting_style: "soft yet dramatic lighting",
                lighting_effect: "creating a mystical and cinematic atmosphere",
                color_palette: "silver, emerald, gold, or deep blue",
                extra_info: "The subject has pointed ears, glowing eyes or ethereal markings, and long flowing hair. The posture conveys elegance and strength. High-fantasy photo style with cinematic detail.",
            },

            Theme::LuxuryTravel => Self {
                pid: "58b57891-31df-440d-8028-8bae3edcb74b",
                theme: "a stylish traveler experiencing high-end destinations",
                attire: "relaxed resort wear, designer pieces, or polished casual looks",
                background_setting: "luxury resorts, scenic coastlines, modern rooftops, private villas, exotic nature retreats, and upscale urban spots",
                background_elements: "breathtaking backdrops, pools, or iconic sites",
                lighting_style: "warm and natural, often during golden hour, sunset, or under clear skies",
                lighting_effect: "creating an aspirational, calm, and confident atmosphere",
                color_palette: "natural, warm, and vibrant colors suitable for vacation spots",
                extra_info: "The subject is posed in leisure moments (by the pool, exploring, etc.). The photography style is editorial, focusing on elegance, adventure, and lifestyle.",
            },

            Theme::GamerGirl => Self {
                pid: "f889ec20-1a01-40f2-a5c3-452193a62b21",
                theme: "a confident gamer immersed in vibrant gaming culture",
                attire: "bold looks like streetwear with techwear accents, casual loungewear with gamer prints, or character-inspired cosplay with accessories like gloves or visors",
                background_setting: "LED-lit gaming rooms, cozy setups, sci-fi inspired spaces, or digital fantasy backdrops",
                background_elements: "PCs, consoles, controllers, and RGB lighting",
                lighting_style: "neon glows, soft ambient RGB, or dramatic shadow-play",
                lighting_effect: "creating an energetic, fashionable, and gaming-forward vibe",
                color_palette: "vibrant neon and RGB colors against darker tones",
                extra_info: "Poses include being seated at a setup, holding a controller, or striking confident, stylized stances. The overall vibe is a blend of realism and virtual aesthetics.",
            },

            Theme::Tinder => Self {
                pid: "fee1d6ae-5c7e-4b37-96b7-f33925733a6e",
                theme: "a candid and genuine lifestyle portrait",
                attire: "a range of outfits from casual smart looks (jeans, neutral tees), to cozy homewear, fitness outfits, or dressed-up evening styles",
                background_setting: "varied real-life settings: walking outside during golden hour, at a café, relaxing at home, or near a car with city or nature backdrops",
                background_elements: "natural elements of the chosen setting",
                lighting_style: "soft and flattering, using natural sunlight or warm indoor tones",
                lighting_effect: "showcasing personality, warmth, and real-life charm without overediting",
                color_palette: "natural and warm color palettes",
                extra_info: "The subject is confident and approachable, with relaxed, smiling, playful, or flirty facial expressions.",
            },

            Theme::Bumble => Self {
                pid: "b5915f61-91d9-42ed-b03c-d01caea19336",
                theme: "a bold and modern lifestyle portrait",
                attire: "clean and elevated fashion: neutral tones, stylish layering, sunglasses, light jackets, or minimalist accessories",
                background_setting: "outdoor brunches, city streets, parks, or during solo adventures",
                background_elements: "elements of a modern, active lifestyle",
                lighting_style: "warm and polished, with natural flares or professional soft tones",
                lighting_effect: "creating a balance of casual charm and self-assured energy",
                color_palette: "clean, modern, and warm palettes",
                extra_info: "Poses focus on openness, strength, and independence, such as smiling with direct eye contact or active motion shots.",
            },

            Theme::Hinge => Self {
                pid: "9cf4f320-b5a4-4747-932c-7ba497db962a",
                theme: "a candid, meaningful, and grounded portrait",
                attire: "casual but well-fitted clothing, like denim, sweaters, simple layers, or clean basics",
                background_setting: "everyday settings like bookstores, kitchens, cozy corners at home, walks in nature, or showing meaningful hobbies in action",
                background_elements: "books, kitchen utensils, cozy home decor, or natural elements",
                lighting_style: "warm, natural, and calming",
                lighting_effect: "creating a relaxed, emotionally open, and authentic mood with soft indoor glow or golden outdoor light",
                color_palette: "natural, warm, and soft tones",
                extra_info: "Expressions are natural and soft: a gentle smile, thoughtful gaze, or subtle humor. The focus is on emotional connection and story.",
            },

            Theme::OkCupid => Self {
                pid: "c6c739eb-b045-4561-87e9-1f849f621aeb",
                theme: "an expressive, quirky, or intellectual portrait",
                attire: "eclectic and unique fashion: funky glasses, patterns, statement pieces, or artsy layers",
                background_setting: "creative and offbeat settings: a record store, vintage café, sketching in a journal, with a pet, or surrounded by books or art",
                background_elements: "records, books, art supplies, or playful decor",
                lighting_style: "artistic lighting, with soft shadows or pops of color",
                lighting_effect: "creating interesting framing and a unique mood",
                color_palette: "eclectic and varied, reflecting a creative personality",
                extra_info: "Expressions are playful, clever, curious, or spontaneous. Each photo showcases identity, depth, and individuality.",
            },

            Theme::FitnessModel => Self {
                pid: "2633ecc0-dda9-419d-b258-3bc1adb167ec",
                theme: "a fit, confident, athletic-inspired portrait",
                attire: "sporty leggings and crop tops, gym shorts, tanks, or sleek activewear sets",
                background_setting: "modern gyms, sunlit rooftops, city parks, or urban streets",
                background_elements: "gym equipment or urban architectural elements",
                lighting_style: "sharp and dynamic, like early morning sun or bright high-key lighting",
                lighting_effect: "creating a clean, motivating, and bold aesthetic that showcases strength and vitality",
                color_palette: "bold, energetic colors or clean, minimalist tones",
                extra_info: "Poses are active and energetic: stretching, walking, laughing, cooling off, or mid-workout moments, exuding energy and self-assurance.",
            },

            Theme::SexySummerBeach => Self {
                pid: "15784aa8-8218-4069-82cc-82db721e67d2",
                theme: "a sun-kissed, playful, and confident beach portrait",
                attire: "swimwear, breezy coverups, sunglasses, or light summer fashion like shorts and tanks",
                background_setting: "a beach or coastal setting, on the sand or in the water",
                background_elements: "the shoreline, waves, sand, umbrellas, or palm trees",
                lighting_style: "warm, golden, and glowing sunlight",
                lighting_effect: "emphasizing natural beauty and creating a flirty, relaxed, and radiant vibe",
                color_palette: "vibrant summer and ocean colors",
                extra_info: "Poses are playful and confident, such as walking along the shoreline, lounging on sand, or laughing in the waves. The mood is full of warm-weather charm and vacation energy.",
            },

            Theme::Cottagecore => Self {
                pid: "aa7fd517-4a07-472b-93c3-12018efd22e7",
                theme: "a gentle, aesthetic, cottagecore-inspired portrait",
                attire: "soft dresses, overalls, linen shirts, vintage cardigans, or straw hats",
                background_setting: "a peaceful countryside or vintage-inspired rural setting, like a meadow, garden, or rustic home",
                background_elements: "wildflowers, rustic home decor, or soft-lit windows",
                lighting_style: "soft, natural, and warm",
                lighting_effect: "creating a nostalgic, calm, and romantic mood",
                color_palette: "muted earth tones",
                extra_info: "Scenes include reading in the grass, baking, picking flowers, or relaxing. The overall mood is cozy and rooted in nature, with whimsical charm.",
            },

            Theme::ModelWalkway => Self {
                pid: "33b39dd6-4ac8-4b68-a6da-71f9b08121ad",
                theme: "a high-fashion runway-style photo",
                attire: "a bold, stylized outfit",
                background_setting: "urban or studio runway-style settings, like minimalist backdrops, city streets, industrial lofts, or high-fashion interiors",
                background_elements: "elements that suggest a runway or high-fashion environment",
                lighting_style: "editorial, crisp, and dramatic",
                lighting_effect: "creating contrast and shadows for a fierce, elevated look",
                color_palette: "bold or minimalist, depending on the fashion",
                extra_info: "Poses are bold and structured: walking straight toward the camera, side glances, turns, and power stances. The result is magazine-worthy.",
            },

            Theme::DoctorHeadshots => Self {
                pid: "76a25ebe-14ab-4800-a2b2-28a68e195fbe",
                theme: "a professional headshot of a confident doctor",
                attire: "a white lab coat and stethoscope",
                background_setting: "a neutral, professional background",
                background_elements: "minimalist, to keep focus on the subject",
                lighting_style: "soft, studio-quality lighting",
                lighting_effect: "creating a realistic and trustworthy portrait",
                color_palette: "neutral and clean tones",
                extra_info: "The expression is confident and approachable. The focus is on realistic facial features.",
            },

            Theme::NurseHeadshots => Self {
                pid: "313a6127-904a-4d32-b379-13823763babb",
                theme: "a professional headshot of a friendly nurse",
                attire: "scrubs or a lab coat, with an optional stethoscope",
                background_setting: "a neutral, professional background",
                background_elements: "minimalist, to keep focus on the subject",
                lighting_style: "soft, studio-quality lighting",
                lighting_effect: "creating a friendly and realistic portrait",
                color_palette: "neutral and clean tones, or colors associated with medical scrubs",
                extra_info: "The expression is friendly and approachable. The focus is on realistic facial features.",
            },

            Theme::SexyNurse => Self {
                pid: "3640d879-3947-458e-ae00-bfd069c841c2",
                theme: "a playful and alluring nurse-themed portrait",
                attire: "a fitted and slightly revealing nurse outfit, with a short dress or scrubs left slightly unbuttoned, holding a stethoscope",
                background_setting: "a cozy indoor or soft bedroom setting",
                background_elements: "soft fabrics and warm ambient elements",
                lighting_style: "soft bedroom lighting",
                lighting_effect: "creating romantic tones and a sultry atmosphere",
                color_palette: "classic white and red, with warm, soft tones",
                extra_info: "The expression is a flirty smile or a sultry side glance, with tousled hair. Studio-quality portrait.",
            },
             Theme::Headshots => Self {
                pid: "ed405897-efae-4f0e-8b0a-917730a2cd1b",
                theme: "a high-resolution, full-body corporate headshot",
                attire: "a sleek, tailored suit (navy, gray, or black) with a white blouse",
                background_setting: "a modern office or a neutral studio setting",
                background_elements: "minimalist and professional, without distractions",
                lighting_style: "soft and professional lighting",
                lighting_effect: "emphasizing sharp details of the attire and expression",
                color_palette: "neutral and corporate tones",
                extra_info: "The subject has a poised yet approachable posture (arms relaxed or lightly crossed) with a slight smile. Hair and makeup are polished. The style is ideal for corporate branding or LinkedIn.",
            },

            Theme::CorporateHeadshots => Self {
                pid: "cd517997-9122-45ca-8b73-cfff446442d2",
                theme: "a professional, AI-generated corporate headshot",
                attire: "modern business attire",
                background_setting: "a well-lit neutral or office-like background",
                background_elements: "clean and professional, suitable for company directories",
                lighting_style: "clear and professional",
                lighting_effect: "creating a soft yet assertive expression with realistic facial features",
                color_palette: "corporate and neutral color schemes",
                extra_info: "Features a symmetrical and centered posture, with shoulders straight and arms naturally placed. The style must be consistent and the framing suitable for websites. All limbs must be correctly rendered.",
            },

            Theme::ModelHeadshots => Self {
                pid: "2b38b63f-f141-4d7a-8f02-49c6bae282c6",
                theme: "a studio-quality fashion model headshot",
                attire: "high-fashion or clean, professional attire",
                background_setting: "a clean white or gray background",
                background_elements: "none, focus is entirely on the model",
                lighting_style: "balanced lighting with soft shadows and a crisp focus",
                lighting_effect: "creating an elegant and confident expression",
                color_palette: "neutral tones that complement the subject's features",
                extra_info: "Features perfect symmetrical face framing, professional makeup, and a sleek hairstyle. The posture is upright with realistic proportions and photorealistic textures. The style is ideal for a modeling portfolio or casting profile.",
            },

            Theme::MobWife => Self {
                pid: "6bc54250-12e5-4365-a263-df4552e72df2",
                theme: "a glamorous portrait channeling the style of a classic mobster's wife",
                attire: "bold prints, faux fur, statement jewelry, and vintage-inspired fashion",
                background_setting: "an elegant yet atmospheric background, like a softly lit parlor, a velvet-draped room, or a vintage-styled lounge",
                background_elements: "elements that suggest richness and mood without being overly detailed",
                lighting_style: "cinematic and realistic lighting",
                lighting_effect: "creating a sharp focus on facial features and a moody atmosphere",
                color_palette: "rich and deep colors, with gold or jewel-tone accents",
                extra_info: "Features a natural posture with a confident expression. The composition is centered. Utmost importance on accurate limb rendering.",
            },

            Theme::Mobster => Self {
                pid: "340c6c6a-567e-4170-88c3-5569ad58f741",
                theme: "a 1940s-era mobster portrait",
                attire: "a dark pinstripe tailored suit with an elegant fedora tilted slightly, accented by a bold gold ring and watch",
                background_setting: "a clean, simple backdrop",
                background_elements: "minimal, to keep focus on the subject",
                lighting_style: "cinematic lighting",
                lighting_effect: "creating a sharp focus on the face and a powerful yet composed expression",
                color_palette: "dark, classic tones like black, navy, and gray with metallic accents",
                extra_info: "Features strong symmetrical shoulders with both arms visible and naturally posed (folded or at sides). The composition is centered in the frame. Requires realistic proportions and detailed textures on fabric and skin.",
            },

            Theme::Cosplay => Self {
                pid: "59b68483-b4e4-4aaa-bc31-f6f94b4dfd60",
                theme: "an imaginative and completely original cosplay character portrait",
                attire: "elaborate and visually striking attire with impressive detail, creatively invented by the AI based on themes like mystical fantasy, sleek sci-fi, grand historical, or whimsical steampunk",
                background_setting: "an artfully blurred, thematically resonant environment that complements the invented character's essence",
                background_elements: "elements that match the unique, invented world of the character",
                lighting_style: "impeccable, professional studio quality",
                lighting_effect: "casting dramatic yet flattering light upon the figure, with a razor-sharp focus on expressive facial features",
                color_palette: "a palette that is dynamically chosen by the AI to fit the invented character's theme",
                extra_info: "CRUCIAL FOR VARIETY: The AI must actively vary the subject's posture (e.g., dynamic action, iconic stances, expressive poses), the invented cosplay character concept, AND the thematic environment in each generation. The subject should be treated as a versatile actor. The composition is a centered medium shot or impactful three-quarter view with a premium cinematic portrait style. UTMOST IMPORTANCE: Ensure all limbs are fully and correctly rendered, anatomically sound, and naturally integrated into a coherent and graceful pose.",
            },
            Theme::SexyCyberpunk => Self {
                pid: "abd58ae0-4b8d-4798-9872-3ebaceeea151",
                theme: "a futuristic boudoir portrait with cyberpunk edge and vaporwave dreams",
                attire: "high-tech cyberpunk lingerie or neon-trimmed bodysuits",
                background_setting: "a glowing cityscape or a holographic interior",
                background_elements: "vaporwave elements like grid lines and chrome textures",
                lighting_style: "ambient neon lighting",
                lighting_effect: "creating a sleek and sophisticated atmosphere",
                color_palette: "soft purples, electric blues, and deep neon highlights",
                extra_info: "The overall mood blends sensuality with digital sophistication. The subject poses with grace, poise, and effortless allure. Critically, ensure all limbs, arms, hands, legs, and feet are clearly visible and anatomically correct.",
            },
            Theme::SexyEaster => Self {
                pid: "9ae527fe-459a-4617-8267-6968e39513c6",
                theme: "a playful and sultry Easter-themed photoshoot",
                attire: "a stylish spring outfit that highlights curves and charm, accented with bunny-inspired accessories",
                background_setting: "an enchanting scene filled with vibrant spring vibes",
                background_elements: "soft pastel colors and decorative elements, without the candy baskets",
                lighting_style: "soft, natural, and flattering",
                lighting_effect: "creating a fresh and flirty atmosphere",
                color_palette: "soft pastels and vibrant spring colors",
                extra_info: "The mood is a blend of playful innocence and sultry flair. The resulting photo should be stylish and flirty.",
            },
            Theme::SexyHalloween => Self {
                pid: "8f1986f7-e2f3-43ca-b4a7-5361601967df",
                theme: "a stunning and spooky Halloween photoshoot",
                attire: "an alluring costume, like a flirtatious witch or a dark-hearted vamp",
                background_setting: "a mysterious and enchanting location",
                background_elements: "magical and spooky elements that enhance the allure",
                lighting_style: "dramatic and moody",
                lighting_effect: "creating a stunning yet dark atmosphere",
                color_palette: "rich, dark tones with vibrant, spooky accents",
                extra_info: "The overall vibe is an equal mix of spooky and stunning, with an enchantress-like allure. The final image should look like a professionally crafted photo, full of digital magic.",
            },
            Theme::Valentine => Self {
                pid: "4fffe9b9-45d6-4564-99b4-055ddb9c90c4",
                theme: "romantic Valentine's Day",
                attire: "an elegant gown in shades of red and pink, with subtle heart motifs and lace details",
                background_setting: "an intimate, candle-lit room",
                background_elements: "soft, cascading rose petals and silky fabrics",
                lighting_style: "warm and soft",
                lighting_effect: "a gentle, romantic glow",
                color_palette: "deep reds, soft pinks, and creamy whites",
                extra_info: "The setting is playful yet classy, evoking charm, allure, and a modern fairytale vibe.",
            },
        }
    }
}
