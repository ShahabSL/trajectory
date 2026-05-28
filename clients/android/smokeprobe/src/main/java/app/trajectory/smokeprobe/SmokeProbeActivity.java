package app.trajectory.smokeprobe;

import android.app.Activity;
import android.graphics.Color;
import android.graphics.Typeface;
import android.graphics.drawable.GradientDrawable;
import android.os.Bundle;
import android.util.Log;
import android.view.Gravity;
import android.view.View;
import android.widget.LinearLayout;
import android.widget.TextView;
import java.io.BufferedReader;
import java.io.File;
import java.io.FileWriter;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import java.nio.charset.StandardCharsets;

public final class SmokeProbeActivity extends Activity {
    private static final String TAG = "TrajectorySmokeProbe";
    private static final String EXTRA_URL = "trajectory_smoke_fetch_url";
    private static final String EXTRA_EXPECT = "trajectory_smoke_expect_body";

    private TextView statusView;
    private TextView detailView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(buildLayout());
        statusView.setText("Probe running");
        statusView.setContentDescription("trajectory.smokeprobe.running");
        detailView.setText("Waiting for tunneled HTTP response");

        String fetchUrl = getIntent().getStringExtra(EXTRA_URL);
        String expectedBody = getIntent().getStringExtra(EXTRA_EXPECT);
        if (fetchUrl == null || fetchUrl.isBlank()) {
            finishWithResult(false, "missing fetch URL");
            return;
        }

        Thread probeThread = new Thread(
            () -> runProbe(fetchUrl, expectedBody == null ? "" : expectedBody),
            "trajectory-smoke-probe"
        );
        probeThread.start();
    }

    private View buildLayout() {
        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setGravity(Gravity.CENTER);
        root.setPadding(44, 44, 44, 44);
        root.setBackgroundColor(Color.rgb(17, 24, 39));

        TextView title = new TextView(this);
        title.setText("Trajectory VPN Smoke Probe");
        title.setTextColor(Color.WHITE);
        title.setTextSize(24);
        title.setTypeface(Typeface.DEFAULT_BOLD);
        title.setGravity(Gravity.CENTER);
        root.addView(title, new LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.MATCH_PARENT,
            LinearLayout.LayoutParams.WRAP_CONTENT
        ));

        addStripe(root, Color.rgb(37, 99, 235));
        addStripe(root, Color.rgb(20, 184, 166));
        addStripe(root, Color.rgb(245, 158, 11));

        LinearLayout panel = new LinearLayout(this);
        panel.setOrientation(LinearLayout.VERTICAL);
        panel.setGravity(Gravity.CENTER);
        panel.setPadding(36, 36, 36, 36);
        GradientDrawable panelBackground = new GradientDrawable();
        panelBackground.setColor(Color.WHITE);
        panelBackground.setCornerRadius(28);
        panelBackground.setStroke(3, Color.rgb(31, 41, 55));
        panel.setBackground(panelBackground);

        statusView = new TextView(this);
        statusView.setGravity(Gravity.CENTER);
        statusView.setTextColor(Color.rgb(17, 24, 39));
        statusView.setTextSize(22);
        statusView.setTypeface(Typeface.DEFAULT_BOLD);
        panel.addView(statusView, new LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.MATCH_PARENT,
            LinearLayout.LayoutParams.WRAP_CONTENT
        ));

        detailView = new TextView(this);
        detailView.setGravity(Gravity.CENTER);
        detailView.setTextColor(Color.rgb(75, 85, 99));
        detailView.setTextSize(16);
        detailView.setPadding(0, 24, 0, 0);
        panel.addView(detailView, new LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.MATCH_PARENT,
            LinearLayout.LayoutParams.WRAP_CONTENT
        ));

        LinearLayout.LayoutParams panelParams = new LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.MATCH_PARENT,
            LinearLayout.LayoutParams.WRAP_CONTENT
        );
        panelParams.setMargins(0, 28, 0, 28);
        root.addView(panel, panelParams);

        addStripe(root, Color.rgb(239, 68, 68));
        addStripe(root, Color.rgb(132, 204, 22));
        addStripe(root, Color.rgb(168, 85, 247));
        return root;
    }

    private void addStripe(LinearLayout root, int color) {
        View stripe = new View(this);
        stripe.setBackgroundColor(color);
        LinearLayout.LayoutParams params = new LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.MATCH_PARENT,
            18
        );
        params.setMargins(0, 10, 0, 0);
        root.addView(stripe, params);
    }

    private void runProbe(String fetchUrl, String expectedBody) {
        try {
            HttpURLConnection connection = (HttpURLConnection) new URL(fetchUrl).openConnection();
            connection.setConnectTimeout(15_000);
            connection.setReadTimeout(15_000);
            connection.setRequestMethod("GET");
            connection.setRequestProperty("Connection", "close");
            int status = connection.getResponseCode();
            InputStream stream = status >= 400 ? connection.getErrorStream() : connection.getInputStream();
            String body = readBody(stream);
            connection.disconnect();
            if (status < 200 || status >= 400) {
                finishWithResult(false, "HTTP status " + status + " body=" + trim(body));
                return;
            }
            if (!expectedBody.isBlank() && !body.contains(expectedBody)) {
                finishWithResult(false, "missing expected marker body=" + trim(body));
                return;
            }
            finishWithResult(true, "HTTP status " + status + " body=" + trim(body));
        } catch (Exception error) {
            finishWithResult(false, error.getClass().getSimpleName() + ": " + error.getMessage());
        }
    }

    private String readBody(InputStream stream) throws Exception {
        if (stream == null) {
            return "";
        }
        StringBuilder body = new StringBuilder();
        try (BufferedReader reader = new BufferedReader(new InputStreamReader(stream, StandardCharsets.UTF_8))) {
            char[] buffer = new char[1024];
            int read;
            while ((read = reader.read(buffer)) != -1 && body.length() < 16_384) {
                body.append(buffer, 0, read);
            }
        }
        return body.toString();
    }

    private void finishWithResult(boolean ok, String detail) {
        String state = ok ? "passed" : "failed";
        String message = "Probe " + state + ": " + detail;
        Log.i(TAG, message);
        writeResult(state + "\n" + detail + "\n");
        runOnUiThread(() -> {
            statusView.setText(message);
            statusView.setContentDescription("trajectory.smokeprobe." + state);
            detailView.setText(ok ? "Separate app traffic crossed the VPN data path." : "Probe failed before tunneled HTTP completed.");
        });
    }

    private void writeResult(String result) {
        try {
            File resultFile = new File(getFilesDir(), "result.txt");
            try (FileWriter writer = new FileWriter(resultFile, false)) {
                writer.write(result);
            }
        } catch (Exception error) {
            Log.w(TAG, "failed to write probe result", error);
        }
    }

    private String trim(String value) {
        if (value == null) {
            return "";
        }
        String compact = value.replace('\n', ' ').replace('\r', ' ').trim();
        if (compact.length() <= 240) {
            return compact;
        }
        return compact.substring(0, 240);
    }
}
