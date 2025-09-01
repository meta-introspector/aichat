## SOP: Setting up Google Gemini OAuth Credentials for AIChat

**Purpose:** This SOP outlines the steps required to obtain Google OAuth 2.0 client credentials and configure them for use with the AIChat application's Gemini API integration.

**Scope:** This procedure applies to users who wish to authenticate AIChat with the Google Gemini API using OAuth 2.0.

**Prerequisites:**
*   Access to a Google Cloud Platform (GCP) account.
*   A GCP project where you have permissions to enable APIs and manage credentials.

**Procedure:**

1.  **Access Google Cloud Console:**
    *   Open your web browser and navigate to the [Google Cloud Console](https://console.cloud.google.com/).
    *   Log in with your Google account.

2.  **Select or Create a GCP Project:**
    *   In the Google Cloud Console, select an existing project from the project dropdown menu at the top of the page.
    *   If you don't have a suitable project, create a new one.

3.  **Enable the Generative Language API:**
    *   In the Google Cloud Console search bar, type "Generative Language API" and select it from the results.
    *   On the API page, click the "ENABLE" button if the API is not already enabled for your project.

4.  **Configure OAuth Consent Screen:**
    *   In the Google Cloud Console, navigate to "APIs & Services" > "OAuth consent screen" (you can use the navigation menu or the search bar).
    *   **User Type:** Choose "External" (if your application will be used by anyone outside your organization) or "Internal" (if only users within your Google Workspace organization will use it). Click "CREATE".
    *   **App registration:**
        *   **App name:** Enter a user-facing name for your application (e.g., "AIChat Gemini Integration").
        *   **User support email:** Select your email address.
        *   **Developer contact information:** Enter your email address.
        *   Click "SAVE AND CONTINUE".
    *   **Scopes:**
        *   On the "Scopes" page, click "ADD OR REMOVE SCOPES".
        *   In the right-hand sidebar, search for and select the following scopes:
            *   `.../auth/cloud-platform`
            *   `.../auth/userinfo.email`
            *   `.../auth/userinfo.profile`
        *   Click "UPDATE".
        *   Click "SAVE AND CONTINUE".
    *   **Test users (if User Type is "External"):** If you selected "External" user type, you will need to add test users who can access your application before it's verified. Add your Google account as a test user.
    *   Review the summary and click "BACK TO DASHBOARD".

5.  **Create OAuth 2.0 Client ID:**
    *   In the Google Cloud Console, navigate to "APIs & Services" > "Credentials".
    *   Click the "+ CREATE CREDENTIALS" button at the top and select "OAuth client ID".
    *   **Application type:** Select "Desktop app".
    *   **Name:** Enter a descriptive name for your client ID (e.g., "AIChat Gemini Desktop Client").
    *   Click "CREATE".

6.  **Download Client Configuration JSON:**
    *   A dialog box will appear displaying your "Client ID" and "Client secret".
    *   Click the "DOWNLOAD JSON" button to download the client configuration file.
    *   Alternatively, you can find your credentials listed under "OAuth 2.0 Client IDs" on the "Credentials" page. Click the download icon next to the client ID you just created.

7.  **Place `client_secret.json` in AIChat Project:**
    *   Rename the downloaded JSON file to `client_secret.json`.
    *   Place this `client_secret.json` file into the `clients/gemini-oauth/` directory within your AIChat project.
    *   The full absolute path to the file should be: `/data/data/com.termux/files/home/storage/github/aichat/clients/gemini-oauth/client_secret.json`.

**Verification:**
*   Ensure the `client_secret.json` file is correctly placed in the specified directory.
*   The file should contain `client_id` and `client_secret` under a `web` key.