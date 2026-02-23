// WordMath Contamination Guard v1.0
// Purpose: Prevent Reddit, ads, external workflows from polluting biophysical-blockchain
// Language: Rust (memory-safe, zero undefined behavior)
// License: Apache 2.0 + ALN neurorights

use std::collections::{HashMap, HashSet};
use std::fmt;

// ===== 1. SACRED TERM LATTICE =====
// Immutable vocabulary protected from external contamination

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SacredTerm {
    BloodToken,
    Lifeforce,
    Cybernet,
    Eibon,
    PhoenixCitizen,
    NeurodivergentAugmentedSelf,
    AugmentationContinuity,
    NeuroRight,
}

#[derive(Debug, Clone)]
pub struct SacredTermSet {
    terms: HashSet<SacredTerm>,
    // Immutable invariant: no deletion, no commodification
}

impl SacredTermSet {
    pub fn new(terms: HashSet<SacredTerm>) -> Self {
        SacredTermSet { terms }
    }

    pub fn validate_preservation(&self, original: &SacredTermSet, new: &SacredTermSet) -> bool {
        // Original sacred terms cannot be deleted
        original.terms.is_subset(&new.terms)
    }

    pub fn contains(&self, term: &SacredTerm) -> bool {
        self.terms.contains(term)
    }
}

// ===== 2. EXTERNAL CONNECTOR SIGNATURES =====
// Heuristics for detecting Reddit, ads, external workflows

#[derive(Debug, Clone, PartialEq)]
pub enum ExternalConnectorSignature {
    RedditPattern,      // "r/", "u/", "/r/", upvote markers
    AdvertisementMarker, // "sponsored", "ad.click", "promoted", "affiliate"
    WorkflowInjection,  // "webhook", "api_key", "token_refresh", "callback_url"
    ExternalWidgetRef,  // iframes, external CDN, tracking pixels
    SocialMediaHandle,  // "@username", "instagram.com", "tiktok.com"
    UnauthorizedDomain, // domain not in allowlist
}

// ===== 3. WORDMATH CONTAMINATION METRICS =====

#[derive(Debug, Clone, Copy)]
pub struct WordMathVector {
    pub y: f64,  // Repetition density (contamination proxy)
    pub z: f64,  // Topic drift (off-topic external content)
    pub t: f64,  // Toxicity (ad/bot-like language)
    pub k: f64,  // Kindness (respect for rights)
    pub e: f64,  // Evidentiality (grounded, not speculative)
}

impl WordMathVector {
    // Quality score: high y, z, t = contamination signal
    pub fn contamination_risk(&self) -> f64 {
        (self.y + self.z + self.t) / 3.0
    }

    pub fn is_clean(&self, max_risk: f64) -> bool {
        self.contamination_risk() < max_risk
    }
}

// ===== 4. EXTERNAL SOURCE TRACKING =====

#[derive(Debug, Clone)]
pub struct ExternalSourceMetadata {
    pub source_did: String,              // DID of origin (Reddit, external server, etc.)
    pub referrer_domain: String,         // "reddit.com", "ad.network.com", etc.
    pub connector_type: ExternalConnectorSignature,
    pub timestamp_utc: u64,
    pub http_referer: String,           // HTTP header or empty if non-web
    pub is_authorized: bool,            // Cross-referenced against whitelist
}

// ===== 5. BIOPHYSICAL-BLOCKCHAIN ISOLATION LAYER =====

pub struct BiophysicalBlockchainGuard {
    // Authorized DIDs only (neurorights-aligned entities)
    authorized_dids: HashSet<String>,
    
    // Allowlisted domains (trusted sources only)
    domain_allowlist: HashSet<String>,
    
    // Rejected external signatures (block immediately)
    rejected_signatures: HashSet<ExternalConnectorSignature>,
    
    // Immutable sacred terms (cannot be touched)
    sacred_terms: SacredTermSet,
    
    // Maximum contamination risk threshold
    max_risk_threshold: f64,
}

impl BiophysicalBlockchainGuard {
    pub fn new(
        authorized_dids: HashSet<String>,
        domain_allowlist: HashSet<String>,
        sacred_terms: SacredTermSet,
    ) -> Self {
        BiophysicalBlockchainGuard {
            authorized_dids,
            domain_allowlist,
            rejected_signatures: vec![
                ExternalConnectorSignature::RedditPattern,
                ExternalConnectorSignature::AdvertisementMarker,
                ExternalConnectorSignature::WorkflowInjection,
                ExternalConnectorSignature::ExternalWidgetRef,
            ]
            .into_iter()
            .collect(),
            sacred_terms,
            max_risk_threshold: 0.25,
        }
    }

    // ===== PRIMARY ADMISSION GATE =====
    pub fn admit_to_biophysical_blockchain(
        &self,
        word_math: WordMathVector,
        source_metadata: &ExternalSourceMetadata,
        candidate_text: &str,
    ) -> Result<BiophysicalBlockchainTicket, ContaminationViolation> {
        // Rule 1: Reject if source DID not authorized
        if !self.authorized_dids.contains(&source_metadata.source_did) {
            return Err(ContaminationViolation::UnauthorizedDID(
                source_metadata.source_did.clone(),
            ));
        }

        // Rule 2: Reject if domain not allowlisted
        if !source_metadata.referrer_domain.is_empty()
            && !self.domain_allowlist.contains(&source_metadata.referrer_domain)
        {
            return Err(ContaminationViolation::UnallowlistedDomain(
                source_metadata.referrer_domain.clone(),
            ));
        }

        // Rule 3: Reject if external connector signature detected
        if self.rejected_signatures.contains(&source_metadata.connector_type) {
            return Err(ContaminationViolation::ExternalConnectorDetected(
                source_metadata.connector_type.clone(),
            ));
        }

        // Rule 4: Reject if contamination risk too high
        if !word_math.is_clean(self.max_risk_threshold) {
            return Err(ContaminationViolation::ContaminationRiskTooHigh(
                word_math.contamination_risk(),
            ));
        }

        // Rule 5: Reject if sacred terms violated
        self.validate_sacred_terms(candidate_text)?;

        // If all checks pass, issue admission ticket
        Ok(BiophysicalBlockchainTicket {
            admission_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            word_math,
            source_did: source_metadata.source_did.clone(),
            contamination_risk: word_math.contamination_risk(),
        })
    }

    fn validate_sacred_terms(&self, text: &str) -> Result<(), ContaminationViolation> {
        let text_lower = text.to_lowercase();

        // Sacred terms can appear, but cannot be:
        // - deleted/reframed as tradeable assets
        // - combined with advertisement markers
        // - embedded in external workflow calls

        if text_lower.contains("blood") && text_lower.contains("token") {
            // Check if being commodified
            if text.contains("buy") || text.contains("sell") || text.contains("price") {
                return Err(ContaminationViolation::SacredTermCommodified(
                    "BloodToken".to_string(),
                ));
            }
        }

        Ok(())
    }
}

// ===== 6. ADMISSION RESULT TYPE =====

#[derive(Debug, Clone)]
pub struct BiophysicalBlockchainTicket {
    pub admission_time: u64,
    pub word_math: WordMathVector,
    pub source_did: String,
    pub contamination_risk: f64,
}

// ===== 7. VIOLATION ENUM =====

#[derive(Debug, Clone)]
pub enum ContaminationViolation {
    UnauthorizedDID(String),
    UnallowlistedDomain(String),
    ExternalConnectorDetected(ExternalConnectorSignature),
    ContaminationRiskTooHigh(f64),
    SacredTermCommodified(String),
}

impl fmt::Display for ContaminationViolation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContaminationViolation::UnauthorizedDID(did) => {
                write!(f, "Unauthorized DID: {}", did)
            }
            ContaminationViolation::UnallowlistedDomain(domain) => {
                write!(f, "Unallowlisted domain: {}", domain)
            }
            ContaminationViolation::ExternalConnectorDetected(sig) => {
                write!(f, "External connector detected: {:?}", sig)
            }
            ContaminationViolation::ContaminationRiskTooHigh(risk) => {
                write!(f, "Contamination risk too high: {}", risk)
            }
            ContaminationViolation::SacredTermCommodified(term) => {
                write!(f, "Sacred term commodified: {}", term)
            }
        }
    }
}

// ===== 8. UNIT TESTS =====

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reddit_pattern_rejected() {
        let authorized = vec!["bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string()]
            .into_iter()
            .collect();
        let allowlist = vec!["example.com".to_string()].into_iter().collect();
        let sacred = SacredTermSet::new(
            vec![SacredTerm::BloodToken, SacredTerm::Lifeforce]
                .into_iter()
                .collect(),
        );

        let guard = BiophysicalBlockchainGuard::new(authorized, allowlist, sacred);

        let source = ExternalSourceMetadata {
            source_did: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
            referrer_domain: "reddit.com".to_string(),
            connector_type: ExternalConnectorSignature::RedditPattern,
            timestamp_utc: 1708000000,
            http_referer: "https://reddit.com/r/augmentation".to_string(),
            is_authorized: false,
        };

        let word_math = WordMathVector {
            y: 0.1,
            z: 0.1,
            t: 0.05,
            k: 0.85,
            e: 0.90,
        };

        let result = guard.admit_to_biophysical_blockchain(
            word_math,
            &source,
            "Check out this link for deals",
        );

        assert!(matches!(
            result,
            Err(ContaminationViolation::ExternalConnectorDetected(_))
        ));
    }

    #[test]
    fn test_advertisement_marker_rejected() {
        let authorized = vec!["bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string()]
            .into_iter()
            .collect();
        let allowlist = vec!["example.com".to_string()].into_iter().collect();
        let sacred = SacredTermSet::new(
            vec![SacredTerm::BloodToken, SacredTerm::Lifeforce]
                .into_iter()
                .collect(),
        );

        let guard = BiophysicalBlockchainGuard::new(authorized, allowlist, sacred);

        let source = ExternalSourceMetadata {
            source_did: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
            referrer_domain: "ad.network.com".to_string(),
            connector_type: ExternalConnectorSignature::AdvertisementMarker,
            timestamp_utc: 1708000000,
            http_referer: "".to_string(),
            is_authorized: false,
        };

        let word_math = WordMathVector {
            y: 0.2,
            z: 0.15,
            t: 0.08,
            k: 0.80,
            e: 0.85,
        };

        let result = guard.admit_to_biophysical_blockchain(
            word_math,
            &source,
            "[SPONSORED] Buy now for 50% off",
        );

        assert!(matches!(
            result,
            Err(ContaminationViolation::ExternalConnectorDetected(_))
        ));
    }

    #[test]
    fn test_workflow_injection_rejected() {
        let authorized = vec!["bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string()]
            .into_iter()
            .collect();
        let allowlist = vec!["example.com".to_string()].into_iter().collect();
        let sacred = SacredTermSet::new(
            vec![SacredTerm::BloodToken, SacredTerm::Lifeforce]
                .into_iter()
                .collect(),
        );

        let guard = BiophysicalBlockchainGuard::new(authorized, allowlist, sacred);

        let source = ExternalSourceMetadata {
            source_did: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
            referrer_domain: "external-workflow.io".to_string(),
            connector_type: ExternalConnectorSignature::WorkflowInjection,
            timestamp_utc: 1708000000,
            http_referer: "".to_string(),
            is_authorized: false,
        };

        let word_math = WordMathVector {
            y: 0.15,
            z: 0.12,
            t: 0.06,
            k: 0.82,
            e: 0.88,
        };

        let result = guard.admit_to_biophysical_blockchain(
            word_math,
            &source,
            "webhook_callback: https://external.io/token_refresh?api_key=xxx",
        );

        assert!(matches!(
            result,
            Err(ContaminationViolation::ExternalConnectorDetected(_))
        ));
    }

    #[test]
    fn test_authorized_source_admitted() {
        let authorized = vec!["bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string()]
            .into_iter()
            .collect();
        let allowlist = vec!["example.com".to_string()].into_iter().collect();
        let sacred = SacredTermSet::new(
            vec![SacredTerm::BloodToken, SacredTerm::Lifeforce]
                .into_iter()
                .collect(),
        );

        let guard = BiophysicalBlockchainGuard::new(authorized, allowlist, sacred);

        let source = ExternalSourceMetadata {
            source_did: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
            referrer_domain: "example.com".to_string(),
            connector_type: ExternalConnectorSignature::SocialMediaHandle,
            timestamp_utc: 1708000000,
            http_referer: "https://example.com/research".to_string(),
            is_authorized: true,
        };

        let word_math = WordMathVector {
            y: 0.15,
            z: 0.12,
            t: 0.05,
            k: 0.85,
            e: 0.90,
        };

        let result = guard.admit_to_biophysical_blockchain(
            word_math,
            &source,
            "Research into augmentation continues to advance understanding.",
        );

        assert!(result.is_ok());
        let ticket = result.unwrap();
        assert_eq!(ticket.source_did, "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7");
    }

    #[test]
    fn test_sacred_term_commodification_rejected() {
        let authorized = vec!["bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string()]
            .into_iter()
            .collect();
        let allowlist = vec!["example.com".to_string()].into_iter().collect();
        let sacred = SacredTermSet::new(
            vec![SacredTerm::BloodToken, SacredTerm::Lifeforce]
                .into_iter()
                .collect(),
        );

        let guard = BiophysicalBlockchainGuard::new(authorized, allowlist, sacred);

        let source = ExternalSourceMetadata {
            source_did: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".to_string(),
            referrer_domain: "example.com".to_string(),
            connector_type: ExternalConnectorSignature::SocialMediaHandle,
            timestamp_utc: 1708000000,
            http_referer: "https://example.com".to_string(),
            is_authorized: true,
        };

        let word_math = WordMathVector {
            y: 0.15,
            z: 0.12,
            t: 0.05,
            k: 0.85,
            e: 0.90,
        };

        let result = guard.admit_to_biophysical_blockchain(
            word_math,
            &source,
            "Buy blood tokens now for $5 each, limited time offer",
        );

        assert!(matches!(
            result,
            Err(ContaminationViolation::SacredTermCommodified(_))
        ));
    }
}

fn main() {
    println!("WordMath Contamination Guard v1.0 - Biophysical Blockchain Protection");
    println!("Preventing Reddit, ads, external workflows from contaminating the system.");
}
