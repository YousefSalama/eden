/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This software may be used and distributed according to the terms of the
 * GNU General Public License version 2.
 */

use mononoke_types::RepositoryId;
use thiserror::Error;

use crate::bubble::BubbleId;

#[derive(Debug, Error)]
pub enum EphemeralBlobstoreError {
    /// The repository does not have an ephemeral blobstore.
    #[error("repo {0} does not have an ephemeral blobstore")]
    NoEphemeralBlobstore(RepositoryId),

    /// A new bubble could not be created.
    #[error("failed to create a new bubble")]
    CreateBubbleFailed,

    /// The requested bubble does not exist.  Either it was never created or has expired.
    #[error("bubble {0} does not exist, or has expired")]
    NoSuchBubble(BubbleId),

    /// An in-use bubble has expired.
    #[error("bubble {0} has expired")]
    BubbleExpired(BubbleId),
}
