import subprocess
import json

proc = subprocess.Popen(["../Engine/game_engine"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
while True:
    game_state_json = proc.stdout.readline()
    game_state = json.loads(game_state_json)
    command = {"action": "move_right"}
    proc.stdin.write(json.dumps(command) + '\n')
    proc.stdin.flush()