### Setup ctehxk2.dll

1. Download and install the [Visual C++ Redistributable Packages for Visual Studio 2013](https://www.microsoft.com/en-us/download/details.aspx?id=40784) for x86.
2. Configure the endpoint to K2 by set the environment variable **K2_BASE_URL** to the corresponding server url, e.g., `http://localhost:8080/k2/ctapi`.



*The following point are needed for a demo setup.*

#### Start *Puppetry* (as *Konnektor*)

1. Build (latest) **puppetry-hall** artifact: `mvn clean package`
2. Extract zip  and adjust config file **conf/puppetryConfig.xml**:
   - puppetryConfig > connectorConfig > httpPort: **8099**
3. Execute `./start.sh`
4. Go to http://127.0.0.1:8080/puppetry/web/
5. Config terminal one of workstation one:
   - add *eGK (ohne Update)*
   - add *Standard SMCB*



#### Start *K2-peak* 

1. Build (latest) **k2-peak** artifact: `mvn clean package`
2. Adjust config file **conf/k2Config.xml**:
   - k2Config > connectorCommunicationConfig > sdsPort: **8099**
   - k2Config > k2CommunicationConfig > host: **0.0.0.0**
   - k2Config > ctapi2ConnectorMappings > ctapi2ConnectorMapping > ctapiTerminalId > no: *empty*
   - k2Config > ctapi2ConnectorMappings > ctapi2ConnectorMapping > ctapiTerminalId > port: *empty*
   - k2Config > ctapi2ConnectorMappings > ctapi2ConnectorMapping > connectorCallContext > terminalId: *empty*
3. Execute `./start.sh`
4. Start (latest) **k2-peak-gui**: `npm i && npm start`

