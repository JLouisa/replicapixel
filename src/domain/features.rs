use std::collections::HashSet;

use chrono::{FixedOffset, Utc};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::Serialize;

use crate::models::{
    FeatureRequestModel, FeatureVoteModel, _entities::sea_orm_active_enums::FeatureStatus,
    feature_request::FeatureRequestModelList, feature_vote::FeatureVoteModelList,
};

#[derive(Clone, Debug, Serialize)]
pub struct FeatureViewList(pub Vec<FeatureView>);
impl FeatureViewList {
    pub fn convert(list: FeatureRequestModelList, votes: FeatureVoteModelList) -> Self {
        let voted_feature_ids: HashSet<i32> =
            votes.0.iter().map(|vote| vote.feature_request_id).collect();
        Self(
            list.0
                .iter()
                .map(|feature| FeatureView::process(feature, &voted_feature_ids))
                .collect(),
        )
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct FeatureView {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub status: FeatureStatus,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub votes: i32,
    pub is_voted: bool,
}
impl FeatureView {
    pub fn process(feature: &FeatureRequestModel, voted_ids_set: &HashSet<i32>) -> Self {
        Self {
            id: feature.id,
            user_id: feature.user_id,
            title: feature.title.clone(),
            description: feature.description.clone(),
            status: feature.status.clone(),
            created_at: feature.created_at,
            updated_at: feature.updated_at,
            votes: feature.votes,
            is_voted: voted_ids_set.contains(&feature.id),
        }
    }
    pub fn process_one(feature: &FeatureRequestModel, voted: bool) -> Self {
        Self {
            id: feature.id,
            user_id: feature.user_id,
            title: feature.title.clone(),
            description: feature.description.clone(),
            status: feature.status.clone(),
            created_at: feature.created_at,
            updated_at: feature.updated_at,
            votes: feature.votes,
            is_voted: voted,
        }
    }
}

impl FeatureVoteModel {
    /// Provides mock vote data specifically for user with ID 1.
    pub fn get_mock_votes_for_user_1() -> FeatureVoteModelList {
        // Use a fixed base time for predictability
        let base_time = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());

        FeatureVoteModelList(vec![
            FeatureVoteModel {
                id: 1,
                user_id: 1,            // Belongs to user 1
                feature_request_id: 1, // Corresponds to "Valentine's Day" feature ID
                created_at: base_time,
                updated_at: base_time,
            },
            FeatureVoteModel {
                id: 2,
                user_id: 1,            // Belongs to user 1
                feature_request_id: 3, // Corresponds to "Professional Head shots" feature ID
                created_at: base_time,
                updated_at: base_time,
            },
            FeatureVoteModel {
                id: 3, // This vote won't match any feature in get_features()
                user_id: 1,
                feature_request_id: 99, // Non-existent feature ID in the mock list
                created_at: base_time,
                updated_at: base_time,
            },
            // Add more votes for user 1 if needed, matching other feature IDs
        ])
    }

    // You could add another function for *all* mock votes if needed for other tests
    // pub fn get_all_mock_votes() -> Vec<FeatureVoteModel> { ... }
}

impl FeatureRequestModel {
    /// Provides a list of mock feature request models.
    pub fn mock_get_features() -> Vec<FeatureRequestModel> {
        // Use a fixed base time for predictability
        let base_time = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
        let later_time = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());

        let sexy_valentine = FeatureRequestModel {
            id: 0, // User 1 hasn't voted for this one in the mock votes
            user_id: 1,
            title: "Sexy Valentine's Day".to_string(),
            description: "Generate images for a spicy Valentine's Day surprise.".to_string(),
            status: FeatureStatus::Suggested,
            votes: 25,
            created_at: base_time,
            updated_at: base_time,
        };
        let valentine_day = FeatureRequestModel {
            id: 1, // User 1 HAS voted for this one (vote id: 1)
            user_id: 2,
            title: "Valentine's Day".to_string(),
            description: "Create romantic imagery suitable for Valentine's Day cards or messages."
                .to_string(),
            status: FeatureStatus::Planned,
            votes: 14,
            created_at: base_time,
            updated_at: later_time, // Example of different update time
        };
        let tinder = FeatureRequestModel {
            id: 2, // User 1 hasn't voted for this one
            user_id: 1,
            title: "Tinder Profile Pics".to_string(),
            description: "Generate profile pictures optimized for dating apps like Tinder."
                .to_string(),
            status: FeatureStatus::Rejected,
            votes: 6,
            created_at: base_time,
            updated_at: base_time,
        };
        let professional_head_shots = FeatureRequestModel {
            id: 3, // User 1 HAS voted for this one (vote id: 2)
            user_id: 4,
            title: "Professional Head shots".to_string(),
            description: "Create professional-looking headshots for LinkedIn or corporate use."
                .to_string(),
            status: FeatureStatus::InProgress,
            votes: 1, // Note: The mock 'votes' field is the total count, not related to user 1's vote specifically
            created_at: later_time,
            updated_at: later_time,
        };

        vec![
            sexy_valentine,
            valentine_day,
            tinder,
            professional_head_shots,
        ]
    }
}
