STEP 1 : Enables and gives a list of file IDs ( script ID in next step )
// kill -s USR1 36092

Input : 
```json
{"id":117, "method": "Debugger.enable"}
``` 

Output : 
```json
// ...
{"method":"Debugger.scriptParsed","params":{"scriptId":"212","url":"file:///Users/piyushgururani/Desktop/Brannch/nodejswatcher/demoapp/routes/home.js","startLine":0,"startColumn":0,"endLine":11,"endColumn":0,"executionContextId":1,"hash":"37d6d64c1fe150a4699986b6182d0fc1608251e3","executionContextAuxData":{"isDefault":true},"isLiveEdit":false,"sourceMapURL":"","hasSourceURL":false,"isModule":false,"length":229}}
//...
{"id":117,"result":{"debuggerId":"(599914EA7194F7CE97C9562B97B14B4)"}}
```

STEP 2 : GET HEAP 

Input : 
```json
    {"id":119, "method": "Runtime.getHeapUsage"  }
```

Output :
```json
    {"id":117,"result":{"usedSize":5629320,"totalSize":7712768}}
```

STEP 3 : Provide file id and line number to this

Input : 
```json
{"id":117, "method": "Debugger.setBreakpoint" , "params" : { "location" : { "scriptId" : "212", "lineNumber" : 6, "columnNumber" : 9 } } }
```

Output : 
```json
    {"id":117,"result":{"breakpointId":"4:6:9:212","actualLocation":{"scriptId":"212","lineNumber":7,"columnNumber":8}}}
```

STEP 3.5 : Called on calling the function 

Output : 
```json
{
  "method": "Debugger.paused",
  "params": {
    "callFrames": [
      {
        "callFrameId": {
          "ordinal": 0,
          "injectedScriptId": 1
        },
        "functionName": "",
        "functionLocation": {
          "scriptId": 212,
          "lineNumber": 4,
          "columnNumber": 16
        },
        "location": {
          "scriptId": 212,
          "lineNumber": 7,
          "columnNumber": 8
        },
        "url": "file:///Users/piyushgururani/Desktop/Brannch/nodejswatcher/demoapp/routes/home.js",
        "scopeChain": [
          {
            "type": "local",
            "object": {
              "type": "object",
              "className": "Object",
              "description": "Object",
              "objectId": {
                "injectedScriptId": 1,
                "id": 2114
              }
            },
            "startLocation": {
              "scriptId": 212,
              "lineNumber": 4,
              "columnNumber": 16
            },
            "endLocation": {
              "scriptId": 212,
              "lineNumber": 8,
              "columnNumber": 1
            }
          },
          {
            "type": "global",
            "object": {
              "type": "object",
              "className": "global",
              "description": "global",
              "objectId": {
                "injectedScriptId": 1,
                "id": 2115
              }
            }
          }
        ],
        "this": {
          "type": "undefined"
        }
      }, 
      {
          /// More frames
      }
    ],
    "reason": "other",
    "hitBreakpoints": [
      "4:6:9:212"
    ]
  }
}
```


STEP 4 : GET VALUE 

Input : 
```json
    {"id":117, "method": "Runtime.getProperties" , "params" : { "objectId" : "{\"injectedScriptId\":1,\"id\":2114}" }  }
```

Output : 
```json
    {"id":117,"result":{"result":[{"name":"req","value":{"type":"object","className":"IncomingMessage","description":"IncomingMessage","objectId":"{\"injectedScriptId\":1,\"id\":2275}"},"writable":true,"configurable":true,"enumerable":true,"isOwn":true},{"name":"res","value":{"type":"object","className":"ServerResponse","description":"ServerResponse","objectId":"{\"injectedScriptId\":1,\"id\":2276}"},"writable":true,"configurable":true,"enumerable":true,"isOwn":true},{"name":"a","value":{"type":"object","subtype":"date","className":"Date","description":"Sat Apr 30 2022 02:48:07 GMT+0530 (India Standard Time)","objectId":"{\"injectedScriptId\":1,\"id\":2277}"},"writable":true,"configurable":true,"enumerable":true,"isOwn":true}]}}
```
 
STEP 5 :

```json
{"id":117, "method": "Debugger.disable"}
``` 
