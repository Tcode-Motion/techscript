# Publishing TechScript to the VS Code Marketplace

To make the TechScript extension searchable and downloadable inside VS Code, Cursor, and other compatible editors, it needs to be published to the **Visual Studio Marketplace**. 

Because this requires a Microsoft Account and an Azure DevOps organization, you will need to do a few steps manually first. Once you have a **Personal Access Token (PAT)**, I can run the final command to publish it for you.

## Step 1: Create a Publisher Account
1. Go to the [Visual Studio Marketplace Management Page](https://marketplace.visualstudio.com/manage).
2. Sign in with your Microsoft account.
3. Click on **Create publisher**.
4. Choose an ID (e.g., `techscript-team` or your own username). We are currently using `techscript-team` in the `package.json`, so if you pick a different ID, let me know so I can update the file!

## Step 2: Get a Personal Access Token (PAT)
VS Code extensions are published via Azure DevOps.
1. Go to your [Azure DevOps Profile](https://dev.azure.com/).
2. Click on the **User settings** icon (top right, next to your profile picture) -> **Personal access tokens**.
3. Click **New Token**.
4. Set the following:
   - **Name**: "VS Code Publishing"
   - **Organization**: `All accessible organizations`
   - **Expiration**: 1 year
   - **Scopes**: Click "Show all scopes" at the bottom, find **Marketplace**, and select **Acquire** and **Manage**.
5. Click **Create** and **Copy the token**. Keep this token safe!

## Step 3: Publish!
Once you have your **Publisher ID** created and your **Personal Access Token** copied, you can tell me:

_"My publisher ID is `<your-id>`. Use this token: `<your-token>`"_

And I will run the command to securely publish the extension to the world!
*(If you prefer to do it yourself to avoid sharing the token, you can open PowerShell in the `vscode-extension` folder and run `npx @vscode/vsce publish -p <your-token>`)*
