Munic Notifications Server Simulator

• General Description
In essence, the Munic simulator serves as a notification server tool that allows you to replay or simulate real-time data. To begin, you have the option of using an existing trip reference file for replaying, or you can upload your own trip file following the naming convention "trip_%i_%yyyy-%mm-%dd.json". After selecting the desired file, specify the URL of your server where you wish to receive the notifications. Once configured, simply initiate the replay process. As a result, you will be able to observe the arrival of POST requests in chronological order, along with their corresponding HTTP response status. Additionally, the simulator provides information such as the timestamp of the most recent request and a brief description of each request.

In the event that your server experiences downtime, there is no need to worry. The Munic simulator has a feature that allows you to store up to 20 packets (configuranle in the .env file SHUTDOWN=true) while waiting for your server to reconnect. This ensures that you receive all the missed data once the connection is reestablished.

Furthermore, we have incorporated a convenient functionality to manage these requests. You have the ability to terminate the current running processes, which is especially useful if you do not wish to wait for the entire replay to finish.

Moving on to the second option, which involves simulating real-time generic and OBD data, the process remains similar. However, you will need to have your own Google Maps API key. If you don't already have one, there is a documentation box that provides a link guiding you through the creation of an API key. Once you have your API key, simply specify a source and destination location, and you are all set to simulate.

If you require more specific field data, you can always utilize the custom fields feature. This feature allows you to add additional fields, choosing from boolean, integer, or random/array strings. These fields can be simulated at a specified frequency of time.

• Requirements : working directory (DIR). You’ll find where to put it inside the .env file.
