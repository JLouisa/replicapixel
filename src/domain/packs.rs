use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Packs {
    id: i32,
    pid: Uuid,
    img: String,
    title: String,
    description: String,
    credits: i32,
    amount: i32,
}
impl Packs {
    pub fn get_packs() -> Vec<Packs> {
        let sexy_valentine = Packs {
            id: 0,
            pid: Uuid::new_v4(),
            img: "https://picsum.photos/id/31/400/500".to_string(),
            title: "Sexy Valentine's Day".to_string(),
            description:
                "Get your partner a special Valentine's Day with this pack of sexy images."
                    .to_string(),
            credits: 100,
            amount: 20,
        };
        let valentine_day = Packs {
            id: 1,
            pid: Uuid::new_v4(),
            img: "https://picsum.photos/id/65/400/500".to_string(),
            title: "Valentine's Day".to_string(),
            description:
                "Get your partner a special Valentine's Day with this pack of romantic images."
                    .to_string(),
            credits: 75,
            amount: 20,
        };
        let tinder = Packs {
            id: 2,
            pid: Uuid::new_v4(),
            img: "https://picsum.photos/id/103/400/500".to_string(),
            title: "Tinder".to_string(),
            description:
                "Get your partner a special Valentine's Day with this pack of Tinder images."
                    .to_string(),
            credits: 65,
            amount: 20,
        };
        let professional_head_shots = Packs {
            id: 3,
            pid: Uuid::new_v4(), img: "https://picsum.photos/id/160/400/500".to_string(),
            title: "Professional Head shots".to_string(),
            description: "Get your partner a special Valentine's Day with this pack of professional head shots.".to_string(),
            credits: 100,
            amount: 20,
        };

        return vec![
            sexy_valentine,
            valentine_day,
            tinder,
            professional_head_shots,
        ];
    }
}
