Version 0.9.1 - 2019-04-25
==========================

  * Update `reqwest` to v0.9.15
  * Add Artist, Track & Album getters to `Scrobbler`

Version 0.9.1 - 2017-11-20
==========================

  * Re-designed authentication API:
    * Add support for authenticating with session key (#16)
    * Implement current password authentication in `Scrobbler::authenticate_with_passsword`
    * Deprecate old `authenticate` method
  * Expose current session key via `Scrobbler::session_key` (#17)
  * Separate API response structs and common values contained in the structs (new 
    `responses::values` package)
    * Use wrapped-vec derived `ScrobbleList` instead of `Vec<ScrobbleResponse>` in various response 
      structs

Version 0.9.0 - 2017-11-16
==========================

  * Batch scrobbling support (#9):
    * Add `Scrobbler::scrobble_batch`
    * Add `ScrobbleBatch` type, auto-generated using `wrapped-vec` crate
    * Add `responses::ScrobbleBatchResponse` response type
  * Scrobble timestamping support (#2)
  * Export `Scrobble` type in crate root; imports for most common use-case more
    ergonomic (#12)
  * Improve `example.rs` readability
  * Various small code improvements


Version 0.3.1 - 2017-10-02
==========================

 * Scrobble implements recommended derivable traits: Clone, PartialEq, Eq,
   PartialOrd, Ord, Hash, Debug. (#11)


Version 0.3.0 - 2017-09-26
==========================

  * New Scrobble struct representing a single track-play (issue #8)
    * Update Scrobbler methods to take Scrobble struct instead of artist & 
      track names as bare Strings.
  * Add support for submitting album data along with track name & artist (#7)


Version 0.2.2 - 2017-09-17
==========================

  * Add timestamp field to ScrobblerResponse (Fixes issue #4)
  * Update to stable Serde v1.0.2
     * Update custom deserializer for CorrectableString
  * Refactor dto.rs into models.rs with sub-packages


Version 0.2.1 - 2017-08-26
==========================

  * Upgrade reqwest dependency to version 0.7.3, fixes reqwest bug with 
    connection pools timing out which broke long-existing Scrobbler instances


Version 0.2.0 - 2017-03-04
==========================

  * Return API responses encoded as structs (Issue #3)
    All Scrobbler methods now return the Last.fm API response deserialized
    into a Rust struct for the type (see API documentation)
  * Add ScrobbleError (with std::err:Error), instead of just returning
    error messages as strings.
  * Refactor Scrobbler/client code:
    * Move request-making code out of Scrobbler and into client; Scrobbler 
      should be an extremely minimal high(est) level layer.
    * Improve internal representation of API methods/operations, using an enum
      instead of passing magic strings around.


Version 0.1.2 - 2017-02-25
==========================

 * Improve API request code:
   * Switch to reqwest instead of hyper for HTTP client
   * Share a HTTP client instance between API requests
   * API request code refactor / cleanup
 * Update serde library dependency to version 0.9


Version 0.1.1 - 2017-02-22
==========================

 * Add API documentation for rustfm-scrobble crate
 * Update data in Cargo.toml


Version 0.1.0 - 2017-02-22
==========================

* Initial release
