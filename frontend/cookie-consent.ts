import "vanilla-cookieconsent/dist/cookieconsent.css";
import * as CookieConsent from "vanilla-cookieconsent";

console.log("[CookieConsent Script] Initializing banner...");

// Define type for potential global variable (optional but good practice)
declare global {
  interface Window {
    consentCookieName?: string;
  }
}
const cookieName = window.consentCookieName || "cc_cookie";

CookieConsent.run({
  revision: 0,
  cookie: {
    name: cookieName,
    expiresAfterDays: 365,
  },

  // Basic GUI setup
  guiOptions: {
    consentModal: {
      layout: "box",
      position: "bottom right",
      equalWeightButtons: true,
    },
    preferencesModal: {
      layout: "box",
      equalWeightButtons: true,
    },
  },

  // --- Callbacks ---
  onFirstConsent: ({ cookie }) => {
    console.log("[CookieConsent Script] onFirstConsent fired. Reloading page...", cookie);
    window.location.reload(); // Reload to let server render correct scripts
  },
  onConsent: ({ cookie }) => {
    console.log("[CookieConsent Script] onConsent fired. Cookie value:", cookie);
    // No tracking initialization needed here
  },
  onChange: ({ cookie, changedCategories }) => {
    console.log(
      "[CookieConsent Script] onChange fired! Changed categories:",
      changedCategories,
      ". Reloading page..."
    );
    window.location.reload(); // Reload to let server render correct scripts
  },

  // --- Define ONLY the categories you use ---
  categories: {
    necessary: {
      enabled: true, // Necessary is always implicitly enabled in opt-in mode
      readOnly: true, // User cannot disable
    },
    analytics: {
      enabled: false, // Start disabled (Opt-in)
      // Optional: Define the service for clarity in preferences modal
      services: {
        ga: {
          label: "Google Analytics",
        },
      },
    },
    marketing: {
      enabled: false,
      services: {
        meta: {
          label: "Meta Pixel (Facebook)",
        },
      },
    },
  },

  // --- Language Configuration (Simplified) ---
  language: {
    default: "en",
    translations: {
      en: {
        consentModal: {
          title: "We use cookies",
          description:
            "We use cookies for essential functions, analytics, and marketing. Choose your preferences or accept all.",
          acceptAllBtn: "Accept all",
          acceptNecessaryBtn: "Accept necessary",
          showPreferencesBtn: "Manage preferences",
          // Ensure these links are correct
          footer: `
            <a href="/policy/privacy" target="_blank" rel="noopener noreferrer">Privacy Policy</a>
            <a href="/policy/cookie" target="_blank" rel="noopener noreferrer">Cookie Policy</a>
          `,
        },
        preferencesModal: {
          title: "Cookie Preferences",
          acceptAllBtn: "Accept all",
          acceptNecessaryBtn: "Accept necessary",
          savePreferencesBtn: "Save preferences",
          closeIconLabel: "Close",
          sections: [
            {
              // Section 1: Optional intro
              title: "Your Choices",
              description:
                "Manage your preferences for different types of cookies used on this site.",
            },
            {
              // Section 2: Necessary
              title: 'Strictly Necessary <span class="pm__badge">Always Enabled</span>',
              description: "Required for the website to function correctly.",
              linkedCategory: "necessary",
            },
            {
              // Section 3: Analytics
              title: "Analytics Cookies",
              description:
                "Help us understand website usage to improve our services (e.g., Google Analytics).",
              linkedCategory: "analytics",
              // You can remove the 'services' definition here if the category title/description is enough
            },
            {
              // Section 4: Marketing
              title: "Marketing Cookies",
              description: "Used to personalize advertising content (e.g., Meta Pixel).",
              linkedCategory: "marketing",
            },
            // Section 5: Optional outro/links
            // {
            //    title: "More Information",
            //    description: `Review our <a href="/cookie-policy">Cookie Policy</a> for details.`
            // }
          ],
        },
      },
      // Add other language translations if needed
    },
  },
});
