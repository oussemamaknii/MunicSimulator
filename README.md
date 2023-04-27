Munic Notifications Server Simulator

• General Description
So basically, Munic simulator is a tool for replaying or simulating real time data as a notification server, so for the first option you just need to upload your tracks/presence files, put your server’s URL, choose dates from which you want to replay, and run it, by that you’ll be able to notify your server with a POST request time sorted and a with HTTP response status indicating if the Paquet have been successfully received or not.

And if in case your server was down for some time don’t you worry, you’ll be able to have up to 10 (adjustable) stored Paquets waiting for your server to reconnect and receive the whole missed data.
Also to mange these requests we added a feature that
Gives you the ability to kill the current running
Processes which makes it easier if you don’t want to
Wait for the whole replay to finish.
Then for the second option which is simulating
real-time generic and obd data, it’s basically the same
process but you just need to have your own Google
maps API KEY which if you don’t there is a little
that shows you a link for creating one, then you add
a source and a destination location, choose a refrence
file, dates and you are all good ready to simulate,
if you want more specefic fields data you are always
welcome to add a more convinent refrence file for your use case.
• Requirements : mongodb URI. You’ll find where to put it inside the .env file.
