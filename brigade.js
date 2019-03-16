const { events, Job, Group } = require("brigadier");
const checkRunImage = "radumatei/brigade-github-check-run:143b5f3";

events.on("check_suite:requested", checkRequested)
events.on("check_suite:rerequested", checkRequested)
events.on("check_run:rerequested", checkRequested)

function checkRequested(e, p) {
    console.log("check requested");
    // Common configuration
    const env = {
        CHECK_PAYLOAD: e.payload,
        CHECK_NAME: "EngineerdCI",
        CHECK_TITLE: "Cargo Build and Cargo Test",
    };

    const build = new Job("rust-build", "rust");
    build.tasks = [
        "git clone https://github.com/engineerd/canonical_json_formatter",
        "cd canonical_json_formatter",
        "cargo build",
        "cargo test"
    ];

    // build.host.name = "virtual-kubelet";
    // build.resourceRequests.cpu = "1";
    // build.resourceRequests.memory = "1G";

    // For convenience, we'll create three jobs: one for each GitHub Check stage.
    const start = new Job("start-run", checkRunImage);
    start.env = env;
    start.env.CHECK_SUMMARY = "Beginning test run";

    // start.host.name = "virtual-kubelet";
    // start.resourceRequests.cpu = "1";
    // start.resourceRequests.memory = "1G";

    const end = new Job("end-run", checkRunImage);
    end.env = env;

    // end.host.name = "virtual-kubelet";
    // end.resourceRequests.cpu = "1";
    // end.resourceRequests.memory = "1G";

    // Now we run the jobs in order:
    // - Notify GitHub of start
    // - Run the test
    // - Notify GitHub of completion
    //
    // On error, we catch the error and notify GitHub of a failure.
    start.run().then(() => {
        return build.run();
    }).then((result) => {
        end.env.CHECK_CONCLUSION = "success";
        end.env.CHECK_SUMMARY = "Build completed";
        end.env.CHECK_TEXT = result.toString();
        return end.run()
    }).catch((err) => {
        // In this case, we mark the ending failed.
        end.env.CHECK_CONCLUSION = "failure";
        end.env.CHECK_SUMMARY = "Build failed";
        end.env.CHECK_TEXT = `Error: ${err}`;
        return end.run();
    });
}
