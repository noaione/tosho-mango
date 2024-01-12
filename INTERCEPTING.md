## Intercepting Network Request

The following is just a general guide to intercept network request, nothing more, nothing less.

### Android

For Android, you would need Rooted Android or an Emulator to get session ID by intercepting network request via tools like [HTTP Toolkit](https://httptoolkit.com/).

You can follow the guide in [HTTP Toolkit](https://httptoolkit.com/docs/guides/android/) on how to intercept HTTPS request.

### Apple

For iOS/Apple, you would need [Stream Network Debug Tool](https://apps.apple.com/us/app/stream-network-debug-tool/id1312141691) to intercept HTTP network

1. Ensure your default browser is Safari for the installation process.
2. Install the app
3. Open the app, click `Sniff now` and a pop-up will appear asking for permission to add VPN configuration, allow it.
4. After you done, go back to the app and you will see another pop-up asking to install CA to sniff HTTPS traffic, allow it.
5. Safari will open asking to allow a download, click `Allow`
6. Open Settings / General / VPN & Device Management, and open the `Stream Generated CA`. Click on it to install the profile.
7. Return to the Stream app and a new pop-up will appear. Click "I've trusted"

