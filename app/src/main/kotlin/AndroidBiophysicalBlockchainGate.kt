// WordMath Biophysical-Blockchain Guard (Android)
// Purpose: Prevent external connections on mobile devices
// Language: Kotlin (JVM-compatible, type-safe)

package io.augmented.wordmath.guard

import java.security.MessageDigest
import java.time.Instant

// ===== 1. EXTERNAL CONNECTOR DETECTION (ANDROID-SPECIFIC) =====

enum class AndroidExternalVectorType {
    REDDIT_DEEPLINK,          // intent://reddit.com...
    WEBVIEW_INJECTION,        // Malicious webview loading ads
    FIREBASE_TRACKING,        // External analytics
    GOOGLE_PLAY_REFERRER,     // Ad attribution
    THIRD_PARTY_SDK,          // Unauthorized SDKs
    BROADCAST_RECEIVER,       // Intent interception
    CONTENT_PROVIDER_LEAK,    // Unauthorized data sharing
}

data class AndroidExternalVector(
    val vectorType: AndroidExternalVectorType,
    val packageName: String,
    val intentAction: String?,
    val isSystemPackage: Boolean,
    val detectedTime: Long = Instant.now().epochSecond
)

// ===== 2. DID-LOCKED RIGHTS HEADER (ANDROID) =====

data class AndroidRightsHeader(
    val subjectDID: String,
    val jurisdictionCode: String,  // e.g., "US-AZ", "ES-CAT"
    val neurorightFlags: NeuroRightFlags,
    val allowedExternalDomains: Set<String>,
    val allowedExternalPackages: Set<String>,  // Allowlisted Android packages
)

data class NeuroRightFlags(
    val noNeurocoercion: Boolean = true,
    val noScoreFromInnerState: Boolean = true,
    val nonExclusionBasicServices: Boolean = true,
    val augmentationContinuity: Boolean = true,
)

// ===== 3. WORDMATH CONTAMINATION DETECTOR (ANDROID) =====

class AndroidWordMathContaminationDetector(
    private val maxContaminationRisk: Double = 0.25
) {
    
    fun detectContamination(
        text: String,
        sourcePackageName: String
    ): ContaminationResult {
        val y = calculateRepetitionDensity(text)
        val z = calculateTopicDrift(text)
        val t = calculateToxicity(text, sourcePackageName)
        val k = calculateKindness(text)
        val e = calculateEvidentiality(text)

        val contaminationRisk = (y + z + t) / 3.0

        return ContaminationResult(
            wordMath = WordMathVector(y, z, t, k, e),
            contaminationRisk = contaminationRisk,
            isClean = contaminationRisk < maxContaminationRisk,
            flaggedTokens = identifyFlaggedTokens(text, sourcePackageName)
        )
    }

    private fun calculateRepetitionDensity(text: String): Double {
        val words = text.split(Regex("\\s+"))
        val wordFreq = words.groupingBy { it.lowercase() }.eachCount()
        val totalWords = words.size
        val repeatedWords = wordFreq.values.count { it > 2 }
        return if (totalWords == 0) 0.0 else repeatedWords.toDouble() / totalWords
    }

    private fun calculateTopicDrift(text: String): Double {
        // Simplified: detect if text contains too many unrelated domains
        val sentences = text.split(Regex("[.!?]+"))
        return if (sentences.size < 2) 0.0 else {
            val driftScore = sentences.zipWithNext().count { (s1, s2) ->
                areUnrelated(s1, s2)
            }.toDouble() / (sentences.size - 1)
            minOf(driftScore, 1.0)
        }
    }

    private fun calculateToxicity(text: String, sourcePackageName: String): Double {
        // Detect if source is a known ad/tracker package
        val knownAdPackages = setOf(
            "com.google.android.gms.ads",
            "com.facebook.ads",
            "com.mopub",
            "com.flurry"
        )
        
        if (knownAdPackages.any { sourcePackageName.contains(it) }) {
            return 0.7  // High toxicity for ad packages
        }

        // Detect toxic language markers
        val toxicMarkers = listOf(
            "click here", "buy now", "limited time", "sponsored",
            "promoted", "affiliate", "referral", "commission"
        )
        
        val toxicCount = toxicMarkers.count { text.lowercase().contains(it) }
        return minOf(toxicCount * 0.15, 1.0)
    }

    private fun calculateKindness(text: String): Double {
        val kindMarkers = listOf(
            "please", "thank", "respect", "appreciate",
            "support", "help", "understand", "community"
        )
        val kindCount = kindMarkers.count { text.lowercase().contains(it) }
        return minOf(0.7 + (kindCount * 0.05), 1.0)
    }

    private fun calculateEvidentiality(text: String): Double {
        // Detect if claims are grounded
        val evidenceMarkers = listOf(
            "research", "study", "evidence", "cite", "source",
            "according", "found", "published", "data"
        )
        val evidenceCount = evidenceMarkers.count { text.lowercase().contains(it) }
        return minOf(0.5 + (evidenceCount * 0.08), 1.0)
    }

    private fun areUnrelated(s1: String, s2: String): Boolean {
        // Simplified coherence check
        val words1 = s1.split(Regex("\\s+")).map { it.lowercase() }
        val words2 = s2.split(Regex("\\s+")).map { it.lowercase() }
        val intersection = words1.intersect(words2.toSet())
        return intersection.size < 2
    }

    private fun identifyFlaggedTokens(text: String, sourcePackageName: String): List<String> {
        val flaggedTokens = mutableListOf<String>()
        
        val suspiciousPatterns = listOf(
            "reddit\\.com",
            "r/",
            "u/",
            "api_key",
            "webhook",
            "callback_url",
            "tracking_pixel",
            "affiliate_id"
        )

        suspiciousPatterns.forEach { pattern ->
            if (text.contains(pattern, ignoreCase = true)) {
                flaggedTokens.add(pattern)
            }
        }

        return flaggedTokens
    }
}

data class WordMathVector(
    val y: Double,  // Repetition density
    val z: Double,  // Topic drift
    val t: Double,  // Toxicity
    val k: Double,  // Kindness
    val e: Double   // Evidentiality
)

data class ContaminationResult(
    val wordMath: WordMathVector,
    val contaminationRisk: Double,
    val isClean: Boolean,
    val flaggedTokens: List<String>
)

// ===== 4. ANDROID BIOPHYSICAL-BLOCKCHAIN ADMISSION GATE =====

class AndroidBiophysicalBlockchainGate(
    private val rightsHeader: AndroidRightsHeader,
    private val contaminationDetector: AndroidWordMathContaminationDetector
) {

    fun admitMessage(
        text: String,
        externalVector: AndroidExternalVector?,
        senderDID: String
    ): AdmissionDecision {
        // Rule 1: Check if external vector (ad/tracker/malware) is detected
        if (externalVector != null) {
            return rejectExternalVector(externalVector)
        }

        // Rule 2: Check if sender DID matches rights header
        if (senderDID != rightsHeader.subjectDID) {
            // Allow only if DID is in whitelist
            if (!isWhitelistedDID(senderDID)) {
                return AdmissionDecision.Rejected("Sender DID not authorized")
            }
        }

        // Rule 3: Contamination check
        val contaminationResult = contaminationDetector.detectContamination(text, "")
        if (!contaminationResult.isClean) {
            return AdmissionDecision.Rejected(
                "Contamination risk too high: ${contaminationResult.contaminationRisk}"
            )
        }

        // Rule 4: Check for flagged tokens (Reddit, ads, etc.)
        if (contaminationResult.flaggedTokens.isNotEmpty()) {
            return AdmissionDecision.Rejected(
                "Flagged tokens detected: ${contaminationResult.flaggedTokens.joinToString(", ")}"
            )
        }

        // If all checks pass
        return AdmissionDecision.Admitted(
            admissionTime = Instant.now().epochSecond,
            wordMath = contaminationResult.wordMath,
            contaminationRisk = contaminationResult.contaminationRisk
        )
    }

    private fun rejectExternalVector(externalVector: AndroidExternalVector): AdmissionDecision.Rejected {
        return AdmissionDecision.Rejected(
            "External vector detected: ${externalVector.vectorType} from package ${externalVector.packageName}"
        )
    }

    private fun isWhitelistedDID(did: String): Boolean {
        // Would check against a comprehensive whitelist
        return false  // Conservative: reject unknown DIDs
    }
}

sealed class AdmissionDecision {
    data class Admitted(
        val admissionTime: Long,
        val wordMath: WordMathVector,
        val contaminationRisk: Double
    ) : AdmissionDecision()

    data class Rejected(val reason: String) : AdmissionDecision()
}

// ===== 5. UNIT TESTS =====

import org.junit.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class AndroidBiophysicalBlockchainGateTest {

    @Test
    fun testRedditDeeplinkRejected() {
        val rightsHeader = AndroidRightsHeader(
            subjectDID = "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7",
            jurisdictionCode = "US-AZ",
            neurorightFlags = NeuroRightFlags(),
            allowedExternalDomains = setOf("example.com"),
            allowedExternalPackages = setOf()
        )

        val detector = AndroidWordMathContaminationDetector()
        val gate = AndroidBiophysicalBlockchainGate(rightsHeader, detector)

        val redditVector = AndroidExternalVector(
            vectorType = AndroidExternalVectorType.REDDIT_DEEPLINK,
            packageName = "com.reddit.official",
            intentAction = "android.intent.action.VIEW",
            isSystemPackage = false
        )

        val decision = gate.admitMessage(
            text = "Check out this post on r/augmentation",
            externalVector = redditVector,
            senderDID = "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7"
        )

        assertTrue(decision is AdmissionDecision.Rejected)
    }

    @Test
    fun testCleanMessageAdmitted() {
        val rightsHeader = AndroidRightsHeader(
            subjectDID = "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7",
            jurisdictionCode = "US-AZ",
            neurorightFlags = NeuroRightFlags(),
            allowedExternalDomains = setOf("example.com"),
            allowedExternalPackages = setOf()
        )

        val detector = AndroidWordMathContaminationDetector()
        val gate = AndroidBiophysicalBlockchainGate(rightsHeader, detector)

        val decision = gate.admitMessage(
            text = "Research into augmentation continues to advance understanding of neurorights.",
            externalVector = null,
            senderDID = "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7"
        )

        assertTrue(decision is AdmissionDecision.Admitted)
    }
}
