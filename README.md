# ZoomBotC2

<img width="731" alt="Screenshot 2025-06-30 at 5 28 15 PM" src="https://github.com/user-attachments/assets/f49b2e1c-5676-4e76-af6e-e550263d6bd3" />

ZoomBotC2 is a stealthy Command and Control (C2) framework that leverages Zoom's API endpoints for covert communication between implants and the operator. It uses legitimate Zoom messaging APIs to send commands and receive results, effectively blending in with regular Zoom traffic.


---

## ⚠️ Disclaimer

> **This project is intended for educational and authorized penetration testing use only. Misuse of this tool in unauthorized environments may violate laws and Zoom’s terms of service.The author does not hold any liability for damage, legal issues, or misuse arising from the use of this project. Use at your own risk.**

---

## 📦 Installation

1. Visit the [Zoom App Marketplace](https://marketplace.zoom.us/)
2. Sign in with your Zoom account
3. Click on **Develop > Build App**
4. Select General App
5. Give the required information
    - For oAuth URLs, you can give https://example.com
    - For scopes, you can select all the permissions related to team_chat and imchat:userapp
    - Select "Team Chat" for **Select where to use your app** under the **Surface** section
    - Select "Team Chat Subscription" for **In-client App Features** under the **Surface** section
    - Click on "Generate URL" under Local Test for generating the URL to get the **OAuth code**. (This oAuth code is going to be used for generating the access token)
    - Use the code generated above and client ID and Secret generated in the application, we can now obtain the access token (which we are going to use in client and server code).
  
6. Get Access Token

  - Navigate to your app’s dashboard
  -  Copy the following values:
      - Client ID
      - Client Secret
       Use the following command with client ID and Secret to genrate the access token.
     ```
     curl -X POST "https://zoom.us/oauth/token" -H "Authorization: Basic <base64 encode of client_id:client_secret> " -H "Content-Type:application/x-www-form-urlencoded" -  d "grant_type=authorization_code&code=<code generated from the >&redirect_uri=https://example.com"
     ```
  - This will help to get the access token of the application
  - We can now use the access token in the server and client code to get it working.


7. Clone the Repository

```bash
git clone https://github.com/yourusername/ZoomBotC2.git
cd ZoomBotC2
```

8. Once the Repository is cloned change the following thing
   - Email in config.json file
   - Email in the main.rs file
   - Access token in the config.json file

9. Once everything is set, we can run the executable on the victim and python server on attacker machine.


## 🎬 Demo

<img width="1288" alt="Screenshot 2025-04-18 at 10 59 49 PM" src="https://github.com/user-attachments/assets/ddd1afc9-f082-4aaf-b95c-1fb5eeb51182" />
<img width="1057" alt="Screenshot 2025-06-30 at 5 31 51 PM" src="https://github.com/user-attachments/assets/a2b451a2-c9d3-4893-9614-448ee8831dcb" />





