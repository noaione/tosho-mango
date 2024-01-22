## Intercepting Network Requests

The following is just a general guide to intercept network requests, nothing more, nothing less.

### Android

For Android, you would need a rooted Android device or an emulator to intercept network requests using tools like [HTTP Toolkit](https://httptoolkit.com/).

You can follow the guide in [HTTP Toolkit](https://httptoolkit.com/docs/guides/android/) on how to intercept HTTPS requests.

### Apple

For iOS/Apple, you would need the [Stream Network Debug Tool](https://apps.apple.com/us/app/stream-network-debug-tool/id1312141691) to intercept HTTP network requests.

1. Ensure your default browser is Safari for the installation process.
2. Install the app.
3. Open the app, click `Sniff now`, and a pop-up will appear asking for permission to add VPN configuration; allow it.
4. After you are done, go back to the app, and you will see another pop-up asking to install a CA to sniff HTTPS traffic; allow it.
5. Safari will open asking to allow a download; click `Allow`.
6. Open Settings / General / VPN & Device Management, and open the `Stream Generated CA`. Click on it to install the profile.
7. Return to the Stream app, and a new pop-up will appear. Click "I've trusted."

### Other mentions

There is another guide by `neckothy` on how to intercept HTTP(s) requests for mobile application, you can find it [here](https://gist.github.com/neckothy/2f4f2a7886953376f080edba0d5a119a).
