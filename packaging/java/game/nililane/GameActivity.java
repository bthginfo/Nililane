package game.nililane;

import android.app.NativeActivity;
import android.content.Context;
import android.hardware.input.InputManager;
import android.media.AudioManager;
import android.os.Build;
import android.view.DisplayCutout;
import android.view.View;
import android.view.WindowInsets;
import android.view.WindowInsetsController;
import android.view.WindowManager;

public final class GameActivity extends NativeActivity
        implements InputManager.InputDeviceListener, View.OnApplyWindowInsetsListener {
    static { System.loadLibrary("nililane"); }

    private static native void resetController(int device);
    private static native void publishInsets(int left, int top, int right, int bottom);
    private static native void publishPhoneEdition(boolean phone);

    private InputManager input;
    private boolean phone;

    @Override
    protected void onCreate(android.os.Bundle state) {
        getWindow().addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
        phone = getPackageName().equals("game.nililane.phone");
        if (phone) publishPhoneEdition(true);
        super.onCreate(state);
        setVolumeControlStream(AudioManager.STREAM_MUSIC);
        input = (InputManager) getSystemService(Context.INPUT_SERVICE);
        getWindow().getDecorView().setOnApplyWindowInsetsListener(this);
    }

    @Override
    protected void onResume() {
        super.onResume();
        if (input != null) input.registerInputDeviceListener(this, null);
    }

    @Override
    protected void onPause() {
        if (input != null) input.unregisterInputDeviceListener(this);
        resetController(-1);
        super.onPause();
    }

    @Override
    public void onWindowFocusChanged(boolean focused) {
        super.onWindowFocusChanged(focused);
        if (!focused) {
            resetController(-1);
            return;
        }
        if (Build.VERSION.SDK_INT >= 30) {
            WindowInsetsController controller = getWindow().getInsetsController();
            if (controller != null) {
                controller.setSystemBarsBehavior(WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
                controller.hide(WindowInsets.Type.systemBars());
            }
        } else {
            getWindow().getDecorView().setSystemUiVisibility(View.SYSTEM_UI_FLAG_FULLSCREEN
                    | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION | View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY);
        }
        getWindow().getDecorView().requestApplyInsets();
    }

    @Override
    public WindowInsets onApplyWindowInsets(View view, WindowInsets insets) {
        DisplayCutout cutout = Build.VERSION.SDK_INT >= 28 ? insets.getDisplayCutout() : null;
        int left = cutout == null ? 0 : cutout.getSafeInsetLeft();
        int top = cutout == null ? 0 : cutout.getSafeInsetTop();
        int right = cutout == null ? 0 : cutout.getSafeInsetRight();
        int bottom = cutout == null ? 0 : cutout.getSafeInsetBottom();
        if (Build.VERSION.SDK_INT >= 29) {
            android.graphics.Insets gestures = insets.getMandatorySystemGestureInsets();
            if (phone) {
                left = Math.max(left, gestures.left);
                right = Math.max(right, gestures.right);
            }
            bottom = Math.max(bottom, gestures.bottom);
        }
        publishInsets(left, top, right, bottom);
        return view.onApplyWindowInsets(insets);
    }

    @Override public void onInputDeviceAdded(int id) {}
    @Override public void onInputDeviceChanged(int id) { resetController(id); }
    @Override public void onInputDeviceRemoved(int id) { resetController(id); }
}
