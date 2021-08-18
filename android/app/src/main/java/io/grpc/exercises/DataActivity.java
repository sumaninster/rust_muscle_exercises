/*
 * Copyright 2016 The gRPC Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package io.grpc.exercises;

import android.content.Context;
import android.os.AsyncTask;
import android.os.Bundle;

import androidx.appcompat.app.AppCompatActivity;

import android.text.TextUtils;
import android.text.method.ScrollingMovementMethod;
import android.view.View;
import android.view.inputmethod.InputMethodManager;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;

import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import io.grpc.StatusRuntimeException;
import io.grpc.muscle.exercises.DataEmpty;
import io.grpc.muscle.exercises.DataGrpc;
import io.grpc.muscle.exercises.DataRequest;
import io.grpc.muscle.exercises.Exercises;
import io.grpc.muscle.exercises.MuscleGroups;

import java.io.PrintWriter;
import java.io.StringWriter;
import java.lang.ref.WeakReference;
import java.text.MessageFormat;
import java.util.Iterator;

public class DataActivity extends AppCompatActivity {
    private EditText hostEdit;
    private EditText portEdit;
    private Button startDataButton;
    private Button exitDataButton;
    private Button getFeatureButton;
    private Button listFeaturesButton;
    private Button recordRouteButton;
    private Button routeChatButton;
    private TextView resultText;
    private ManagedChannel channel;
    private EditText requestDataId;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_data);
        hostEdit = (EditText) findViewById(R.id.host_edit_text);
        portEdit = (EditText) findViewById(R.id.port_edit_text);
        startDataButton = (Button) findViewById(R.id.start_route_guide_button);
        exitDataButton = (Button) findViewById(R.id.exit_route_guide_button);
        getFeatureButton = (Button) findViewById(R.id.get_feature_button);
        listFeaturesButton = (Button) findViewById(R.id.list_features_button);
        recordRouteButton = (Button) findViewById(R.id.record_route_button);
        routeChatButton = (Button) findViewById(R.id.route_chat_button);
        requestDataId = (EditText) findViewById(R.id.request_data_id);
        resultText = (TextView) findViewById(R.id.result_text);
        resultText.setMovementMethod(new ScrollingMovementMethod());
        disableButtons();
    }

    public void startData(View view) {
        String host = hostEdit.getText().toString();
        String portStr = portEdit.getText().toString();
        int port = TextUtils.isEmpty(portStr) ? 0 : Integer.parseInt(portStr);
        ((InputMethodManager) getSystemService(Context.INPUT_METHOD_SERVICE))
                .hideSoftInputFromWindow(hostEdit.getWindowToken(), 0);
        channel = ManagedChannelBuilder.forAddress(host, port).usePlaintext().build();
        hostEdit.setEnabled(false);
        portEdit.setEnabled(false);
        startDataButton.setEnabled(false);
        enableButtons();
    }

    public void exitData(View view) {
        channel.shutdown();
        disableButtons();
        hostEdit.setEnabled(true);
        portEdit.setEnabled(true);
        startDataButton.setEnabled(true);
    }

    public void getAllMuscleGroups(View view) {
        setResultText("");
        disableButtons();
        new GrpcTask(new AllMuscleGroups(), channel, this).execute();
    }

    public void getAllExercises(View view) {
        setResultText("");
        disableButtons();
        new GrpcTask(new AllExercises(), channel, this).execute();
    }

    public void getExercisesForMuscleGroup(View view) {
        setResultText("");
        disableButtons();
        new GrpcTask(new ExercisesForMuscleGroup(getDataId()), channel, this).execute();
    }

    public void getMuscleGroupsForExercise(View view) {
        setResultText("");
        disableButtons();
        new GrpcTask(new MuscleGroupsForExercise(getDataId()), channel, this).execute();
    }

    private long getDataId() {
        if (requestDataId.getText().toString().trim().isEmpty()) return 1;
        return Long.parseLong(requestDataId.getText().toString());
    }

    private void setResultText(String text) {
        resultText.setText(text);
    }

    private void disableButtons() {
        getFeatureButton.setEnabled(false);
        listFeaturesButton.setEnabled(false);
        recordRouteButton.setEnabled(false);
        routeChatButton.setEnabled(false);
        exitDataButton.setEnabled(false);
        requestDataId.setEnabled(false);
    }

    private void enableButtons() {
        exitDataButton.setEnabled(true);
        getFeatureButton.setEnabled(true);
        listFeaturesButton.setEnabled(true);
        recordRouteButton.setEnabled(true);
        routeChatButton.setEnabled(true);
        requestDataId.setEnabled(true);
    }

    private static class GrpcTask extends AsyncTask<Void, Void, String> {
        private final GrpcRunnable grpcRunnable;
        private final ManagedChannel channel;
        private final WeakReference<DataActivity> activityReference;

        GrpcTask(GrpcRunnable grpcRunnable, ManagedChannel channel, DataActivity activity) {
            this.grpcRunnable = grpcRunnable;
            this.channel = channel;
            this.activityReference = new WeakReference<>(activity);
        }

        @Override
        protected String doInBackground(Void... nothing) {
            try {
                String logs =
                        grpcRunnable.run(
                                DataGrpc.newBlockingStub(channel), DataGrpc.newStub(channel));
                return "Success!\n" + logs;
            } catch (Exception e) {
                StringWriter sw = new StringWriter();
                PrintWriter pw = new PrintWriter(sw);
                e.printStackTrace(pw);
                pw.flush();
                return "Failed... :\n" + sw;
            }
        }

        @Override
        protected void onPostExecute(String result) {
            DataActivity activity = activityReference.get();
            if (activity == null) {
                return;
            }
            activity.setResultText(result);
            activity.enableButtons();
        }
    }

    private interface GrpcRunnable {
        String run(DataGrpc.DataBlockingStub blockingStub, DataGrpc.DataStub asyncStub) throws Exception;
    }

    private static class AllMuscleGroups implements GrpcRunnable {
        @Override
        public String run(DataGrpc.DataBlockingStub blockingStub, DataGrpc.DataStub asyncStub) {
            return allMuscleGroups(blockingStub);
        }

        private String allMuscleGroups(
                DataGrpc.DataBlockingStub blockingStub)
                throws StatusRuntimeException {
            StringBuffer logs = new StringBuffer("Result: ");
            appendLogs(
                    logs,
                    "*** allMuscleGroups: ***");

            DataEmpty request = DataEmpty.newBuilder().build();
            Iterator<MuscleGroups> muscleGroups;
            muscleGroups = blockingStub.allMuscleGroups(request);

            while (muscleGroups.hasNext()) {
                MuscleGroups muscleGroup = muscleGroups.next();
                appendLogs(logs, muscleGroup.toString());
            }
            return logs.toString();
        }
    }

    private static class AllExercises implements GrpcRunnable {
        @Override
        public String run(DataGrpc.DataBlockingStub blockingStub, DataGrpc.DataStub asyncStub) {
            return allExercises(blockingStub);
        }

        private String allExercises(
                DataGrpc.DataBlockingStub blockingStub)
                throws StatusRuntimeException {
            StringBuffer logs = new StringBuffer("Result: ");
            appendLogs(
                    logs,
                    "*** allMuscleGroups: ***");

            DataEmpty request = DataEmpty.newBuilder().build();
            Iterator<Exercises> exercises;
            exercises = blockingStub.allExercises(request);

            while (exercises.hasNext()) {
                Exercises exercise = exercises.next();
                appendLogs(logs, exercise.toString());
            }
            return logs.toString();
        }
    }

    private static class ExercisesForMuscleGroup implements GrpcRunnable {
        private final long id;

        ExercisesForMuscleGroup(long _id) {
            id = _id;
        }

        @Override
        public String run(DataGrpc.DataBlockingStub blockingStub, DataGrpc.DataStub asyncStub) {
            return exercisesForMuscleGroup(id, blockingStub);
        }

        private String exercisesForMuscleGroup(
                long id, DataGrpc.DataBlockingStub blockingStub)
                throws StatusRuntimeException {
            StringBuffer logs = new StringBuffer("Result: ");
            appendLogs(
                    logs,
                    "*** exercisesForMuscleGroup: Muscle Group Id={0}", id);

            DataRequest request =
                    DataRequest.newBuilder()
                            .setId(id)
                            .build();
            Iterator<Exercises> exercises;
            exercises = blockingStub.exercisesForMuscleGroup(request);

            while (exercises.hasNext()) {
                Exercises exercise = exercises.next();
                appendLogs(logs, exercise.toString());
            }
            return logs.toString();
        }
    }

    private static class MuscleGroupsForExercise implements GrpcRunnable {
        private final long id;

        MuscleGroupsForExercise(long _id) {
            id = _id;
        }

        @Override
        public String run(DataGrpc.DataBlockingStub blockingStub, DataGrpc.DataStub asyncStub) {
            return muscleGroupsForExercise(id, blockingStub);
        }

        private String muscleGroupsForExercise(
                long id, DataGrpc.DataBlockingStub blockingStub)
                throws StatusRuntimeException {
            StringBuffer logs = new StringBuffer("Result: ");
            appendLogs(
                    logs,
                    "*** muscleGroupsForExercise: Exercise Id={0} ***", id);

            DataRequest request = DataRequest.newBuilder().setId(id).build();
            Iterator<MuscleGroups> muscleGroups;
            muscleGroups = blockingStub.muscleGroupsForExercise(request);

            while (muscleGroups.hasNext()) {
                MuscleGroups muscleGroup = muscleGroups.next();
                appendLogs(logs, muscleGroup.toString());
            }
            return logs.toString();
        }
    }

    private static void appendLogs(StringBuffer logs, String msg, Object... params) {
        if (params.length > 0) {
            logs.append(MessageFormat.format(msg, params));
        } else {
            logs.append(msg);
        }
        logs.append("\n");
    }
}
