use dioxus::prelude::*;

#[component]
pub fn TermsOfService() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-8 max-w-4xl",
            h1 { class: "text-4xl font-bold mb-8", "Terms of Service" }
            div { class: "prose prose-lg",
                h2 { class: "text-2xl font-bold mt-6 mb-4", "1. ACCEPTANCE OF TERMS" }
                p {
                    "By accessing and using GetTheAlbum.com (\"the Website\"), you agree to comply with and be bound by these Terms of Use and Copyright Policy. If you do not agree with these terms, you should not use this Website."
                }

                h2 { class: "text-2xl font-bold mt-6 mb-4", "2. INTELLECTUAL PROPERTY RIGHTS" }
                h3 { class: "text-xl font-semibold mt-4 mb-2", "2.1 Ownership of Content" }
                p {
                    "All content, including but not limited to music tracks, lyrics, blog posts, images, videos, logos, and software (collectively, \"Content\"), is the property of Get The Album LLC (\"the Owner\") and is protected by copyright, trademark, and other intellectual property laws. Unauthorized use, reproduction, distribution, or modification of the Content is strictly prohibited without prior written permission."
                }

                h3 { class: "text-xl font-semibold mt-4 mb-2", "2.2 Licensing & User Rights" }
                p {
                    "Free Music Downloads & Streams: The Website may provide free access to music tracks, which are licensed for personal, non-commercial use only."
                }
                p {
                    "Paid Features & Memberships: Certain features may require payment or subscription, which grants users a limited, non-transferable, and non-exclusive license to access the specified content."
                }
                p {
                    "User-Generated Content (UGC): If you submit comments, feedback, or other contributions, you grant the Website a non-exclusive, worldwide, royalty-free license to use, display, and distribute your content."
                }

                h2 { class: "text-2xl font-bold mt-6 mb-4",
                    "3. USER COMMENTS AND COMMUNITY GUIDELINES"
                }
                h3 { class: "text-xl font-semibold mt-4 mb-2", "3.1 Commenting & User Interactions" }
                p {
                    "Users may participate in discussions via blog comments and other interactive features. By posting comments, you agree:"
                }
                ul { class: "list-disc pl-4 mt-2",
                    li {
                        "Not to submit content that is illegal, defamatory, obscene, or infringes on the rights of others."
                    }
                    li { "Not to spam, harass, or engage in abusive behavior." }
                    li {
                        "That the Website reserves the right to moderate, remove, or disable comments at its discretion."
                    }
                }

                h3 { class: "text-xl font-semibold mt-4 mb-2",
                    "3.2 Copyright Infringement in Comments"
                }
                p {
                    "Users may not post, link, or distribute copyrighted material without authorization. If you believe that content posted by another user infringes your copyright, please follow the procedure in Section 6: Copyright Complaints (DMCA Policy)."
                }

                h2 { class: "text-2xl font-bold mt-6 mb-4", "4. PAID FEATURES AND SUBSCRIPTIONS" }
                h3 { class: "text-xl font-semibold mt-4 mb-2", "4.1 Payments & Refunds" }
                p {
                    "Access to premium content, exclusive material, or membership plans may require payment."
                }
                p {
                    "All payments are final, and refunds will only be issued in cases where the service was not delivered as promised."
                }
                p {
                    "If you experience billing issues, contact getthealbum@protonmail.com within 30 days of the transaction."
                }

                h3 { class: "text-xl font-semibold mt-4 mb-2", "4.2 User Account Responsibilities" }
                ul { class: "list-disc pl-4 mt-2",
                    li { "Users are responsible for maintaining the security of their accounts." }
                    li { "Sharing of account credentials is prohibited." }
                    li {
                        "The Website reserves the right to terminate accounts that violate these terms."
                    }
                }

                h2 { class: "text-2xl font-bold mt-6 mb-4",
                    "5. COPYRIGHT POLICY & LICENSE RESTRICTIONS"
                }
                h3 { class: "text-xl font-semibold mt-4 mb-2", "5.1 Permitted Use" }
                p {
                    "Users may stream and download tracks where permitted but may not modify, sell, or redistribute them without prior written permission."
                }
                p {
                    "Blogs and educational material may be shared for personal use but cannot be copied or rehosted commercially."
                }

                h3 { class: "text-xl font-semibold mt-4 mb-2", "5.2 Restrictions" }
                ul { class: "list-disc pl-4 mt-2",
                    li {
                        "Commercial use of the Website's music, blog posts, and media requires explicit licensing agreements."
                    }
                    li {
                        "Use of content for AI training, data mining, or any automated process is strictly prohibited."
                    }
                    li { "No scraping, framing, or unauthorized embedding of Website content." }
                }

                h2 { class: "text-2xl font-bold mt-6 mb-4", "6. COPYRIGHT COMPLAINTS (DMCA POLICY)" }
                h3 { class: "text-xl font-semibold mt-4 mb-2", "6.1 Filing a Copyright Complaint" }
                p {
                    "If you believe your copyrighted material has been used without authorization, you may file a Digital Millennium Copyright Act (DMCA) takedown request by providing the following information:"
                }
                ul { class: "list-disc pl-4 mt-2",
                    li { "A description of the copyrighted work." }
                    li { "The exact URL where the infringing material is located." }
                    li { "Your contact information (name, email, and mailing address)." }
                    li { "A statement that you have a good faith belief that the use is unauthorized." }
                    li {
                        "A statement that the information provided is accurate under penalty of perjury."
                    }
                    li { "Your signature (electronic or physical)." }
                }
                p { "Send this information to: getthealbum@protonmail.com" }

                h3 { class: "text-xl font-semibold mt-4 mb-2", "6.2 Counter-Notification Process" }
                p {
                    "If your content was removed due to a copyright complaint, and you believe this was an error, you may file a counter-notification with:"
                }
                ul { class: "list-disc pl-4 mt-2",
                    li { "Identification of the removed material and its prior location." }
                    li {
                        "A statement under penalty of perjury that you believe the removal was a mistake."
                    }
                    li { "Consent to jurisdiction in the courts of New York, Wyoming, Delaware." }
                }
                p { "Submit counter-notifications to getthealbum@protonmail.com." }

                h2 { class: "text-2xl font-bold mt-6 mb-4", "7. LIMITATION OF LIABILITY" }
                ul { class: "list-disc pl-4 mt-2",
                    li {
                        "The Website and its owners shall not be liable for any damages resulting from the use or inability to use the Website."
                    }
                    li { "The Website does not guarantee uninterrupted availability of content." }
                    li { "Users agree that their use of the Website is at their own risk." }
                }

                h2 { class: "text-2xl font-bold mt-6 mb-4", "8. CHANGES TO THESE TERMS" }
                p {
                    "We reserve the right to update these Terms at any time. Changes take effect upon posting on this page. Continued use of the Website after updates constitutes acceptance of the new terms."
                }

                h2 { class: "text-2xl font-bold mt-6 mb-4", "9. CONTACT INFORMATION" }
                p { "If you have questions about these Terms or need legal assistance, contact:" }
                p { "getthealbum@protonmail.com" }
            }
        }
    }
}
