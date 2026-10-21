import importlib.util
from pathlib import Path
import unittest


spec = importlib.util.spec_from_file_location("package_version", Path(__file__).with_name("package-version.py"))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
package_version = module.package_version


class PackageVersionTests(unittest.TestCase):
    def test_master_uses_the_existing_workflow_run_counter(self):
        env = {"GITHUB_REF": "refs/heads/master", "GITHUB_RUN_NUMBER": "815"}
        self.assertEqual(package_version("6.5", env), "6.5.815")
        self.assertEqual(package_version("6.6", env), "6.6.815")

    def test_each_master_run_gets_a_new_stable_version(self):
        for run in (815, 816):
            env = {"GITHUB_REF": "refs/heads/master", "GITHUB_RUN_NUMBER": str(run)}
            self.assertEqual(package_version("6.5", env), f"6.5.{run}")

    def test_pull_request_from_a_fork_master_is_a_prerelease(self):
        env = {"GITHUB_REF": "refs/pull/12/merge", "GITHUB_HEAD_REF": "master", "GITHUB_RUN_NUMBER": "817"}
        self.assertEqual(package_version("6.5", env), "6.5.817-branch-master")

    def test_numeric_and_punctuation_branch_names_are_valid_prereleases(self):
        for branch, expected in [("001", "001"), ("Feature/New Sensor", "feature-new-sensor"), ("///", "dev")]:
            env = {"GITHUB_REF_NAME": branch, "GITHUB_RUN_NUMBER": "818"}
            self.assertEqual(package_version("6.5", env), f"6.5.818-branch-{expected}")

    def test_reruns_keep_the_same_version(self):
        env = {"GITHUB_REF": "refs/heads/master", "GITHUB_RUN_NUMBER": "815", "GITHUB_RUN_ATTEMPT": "2"}
        self.assertEqual(package_version("6.5", env), "6.5.815")

    def test_local_builds_cannot_be_mistaken_for_releases(self):
        self.assertEqual(package_version("6.5", {}), "6.5.0-local")

    def test_invalid_base_and_counter_are_rejected(self):
        for base in ("6.5.0", "v6.5", "06.5", "6.5-rc", "6"):
            with self.subTest(base=base), self.assertRaises(ValueError):
                package_version(base, {})
        for run in ("0", "0815", "-1", "815.2"):
            with self.subTest(run=run), self.assertRaises(ValueError):
                package_version("6.5", {"GITHUB_RUN_NUMBER": run})


if __name__ == "__main__":
    unittest.main()
