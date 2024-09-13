use crate::client::LichessApi;
use crate::error::Result;
use crate::model::{users::*, LightUser};

impl LichessApi<reqwest::Client> {
    pub async fn get_public_user_data(
        &self,
        request: impl Into<public::GetRequest>,
    ) -> Result<UserExtended> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_status_of_users(
        &self,
        request: impl Into<status::GetRequest>,
    ) -> Result<Vec<status::User>> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_rating_history(
        &self,
        request: impl Into<rating_history::GetRequest>,
    ) -> Result<rating_history::RatingHistory> {
        self.get_single_model(request.into()).await
    }

    /// Get performance statistics of a user.
    pub async fn get_user_performance_statistics(
        &self,
        request: impl Into<performance::GetRequest>,
    ) -> Result<performance::Performance> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_users_by_id(
        &self,
        request: impl Into<by_id::PostRequest>,
    ) -> Result<Vec<User>> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_live_streamers(&self) -> Result<Vec<StreamingUser>> {
        self.get_single_model(live_streamers::GetRequest::new())
            .await
    }

    pub async fn get_crosstable(
        &self,
        request: impl Into<crosstable::GetRequest>,
    ) -> Result<Crosstable> {
        self.get_single_model(request.into()).await
    }

    /// Get user autocomplete results.
    ///
    /// This differs from [`LichessApi::autocomplete_usernames`] by not returning user information
    /// and not only the usernames.
    pub async fn autocomplete_users(
        &self,
        request: impl Into<autocomplete::GetRequest>,
    ) -> Result<Vec<LightUser>> {
        self.get_single_model(request.into()).await
    }

    /// Get username autocomplete results.
    ///
    /// This differs from [`LichessApi::autocomplete_users`] by not returning any user information
    /// aside from the username itself.
    pub async fn autocomplete_usernames(
        &self,
        request: impl Into<autocomplete_name::GetRequest>,
    ) -> Result<Vec<String>> {
        self.get_single_model(request.into()).await
    }
}
