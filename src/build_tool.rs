use crate::cache::Cache;
use crate::rust::Project;
use crate::{Lang, DEBUG};
use std::process::Command;
use std::fs;

pub fn build_tool(lang: &Lang, command_str: &str, cache: &mut Cache) -> (bool, String) {
    match lang {
        Lang::Rust => {
            println!("Launch: {}", command_str);
            let code = fs::read_to_string("sandbox/src/lib.rs").expect("Failed to read sandbox/src/lib.rs. Make sure the file exists.");
            let dependencies = fs::read_to_string("sandbox/Cargo.toml").expect("Failed to read sandbox/Cargo.toml. Make sure the file exists.");
            let src = format!("{}\n{}", dependencies, code);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts = command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    println!("Running command: {} with args: {:?}", command_parts[0], args);

                    let output = match Command::new(command_parts[0])
                        .args(args)
                        .current_dir("sandbox")
                        .output()
                    {
                        Ok(output) => output,
                        Err(e) => {
                            eprintln!("Failed to execute command: {}", e);
                            return (false, format!("Failed to execute command: {}", e));
                        }
                    };

                    let exit_code = output.status.code().unwrap_or(1);
                    let std_err = String::from_utf8(output.stderr).unwrap_or_else(|_| "Failed to read stderr".to_string());
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap_or_else(|_| "Failed to serialize output".to_string());
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => result.to_string(),
            };

            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap_or_else(|_| (1, "Failed to parse result".to_string()));
            (parsed.0 == 0, parsed.1)
        }
        Lang::Java => {
            println!("Launch: {}", command_str);
            let code =
                std::fs::read_to_string("sandbox/src/main/java/com/example/solution/Solution.java")
                    .unwrap();
            let test = std::fs::read_to_string(
                "sandbox/src/test/java/com/example/solution/SolutionTest.java",
            )
            .unwrap();
            let code_and_test = format!("{}\n{}", code, test);
            let dependencies = std::fs::read_to_string("sandbox/pom.xml").unwrap();
            let src = format!("{}\n{}", dependencies, code_and_test);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts = command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    // check OS if windows then add ".cmd" to command name in command_parts[0]
                    let command = if cfg!(target_os = "windows") {
                        format!("{}.cmd", command_parts[0])
                    } else {
                        command_parts[0].to_string()
                    };
                    let output = std::process::Command::new(command)
                        .args(args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();
                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => result.to_string(),
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }

        Lang::Scala => {
            println!("Launch: {}", command_str);
            let code = std::fs::read_to_string("sandbox/src/main/scala/Solution.scala").unwrap();
            let test =
                std::fs::read_to_string("sandbox/src/test/scala/SolutionTest.scala").unwrap();
            let code_and_test = format!("{}\n{}", code, test);
            let dependencies = std::fs::read_to_string("sandbox/build.sbt").unwrap();
            let src = format!("{}\n{}", dependencies, code_and_test);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts = command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    // check OS if windows then add ".cmd" to command name in command_parts[0]
                    let command = if cfg!(target_os = "windows") {
                        format!("{}.cmd", command_parts[0])
                    } else {
                        command_parts[0].to_string()
                    };
                    let output = std::process::Command::new(command)
                        .args(args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();
                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => result.to_string(),
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }
        Lang::Swift => {
            println!("Launch: {}", command_str);
            let code = std::fs::read_to_string("sandbox/Sources/Solution/Solution.swift").unwrap();
            let test =
                std::fs::read_to_string("sandbox/Tests/SolutionTests/SolutionTests.swift").unwrap();
            let code_and_test = format!("{}\n{}", code, test);
            let dependencies = std::fs::read_to_string("sandbox/Package.swift").unwrap();
            let src = format!("{}\n{}", dependencies, code_and_test);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts = command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    // check OS if windows then add ".cmd" to command name in command_parts[0]
                    let command = command_parts[0].to_string();
                    let output = std::process::Command::new(command)
                        .args(args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();
                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => result.to_string(),
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }
        Lang::Kotlin => {
            println!("Launch: {}", command_str);
            let code = std::fs::read_to_string("sandbox/src/main/kotlin/Solution.kt").unwrap();
            let test = std::fs::read_to_string("sandbox/src/test/kotlin/SolutionTest.kt").unwrap();
            let code_and_test = format!("{}\n{}", code, test);
            let dependencies = std::fs::read_to_string("sandbox/build.gradle").unwrap();
            let src = format!("{}\n{}", dependencies, code_and_test);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts = command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    // check OS if windows then add ".cmd" to command name in command_parts[0]
                    let command = if cfg!(target_os = "windows") {
                        format!("{}.bat", command_parts[0])
                    } else {
                        command_parts[0].to_string()
                    };
                    // println!("{}, {:?}", command, args);
                    let output = std::process::Command::new(command)
                        .args(args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();
                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => result.to_string(),
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }

        Lang::Python => {
            println!("Launch: {}", command_str);
            if command_str == "" {
                return (true, "".to_string());
            }
            let code = std::fs::read_to_string("sandbox/solution.py").unwrap();
            let test = std::fs::read_to_string("sandbox/test.py").unwrap();
            let code_and_test = format!("{}\n{}", code, test);
            let dependencies = std::fs::read_to_string("sandbox/requirements.txt").unwrap();
            let src = format!("{}\n{}", dependencies, code_and_test);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts = command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    let output = std::process::Command::new(command_parts[0])
                        .args(args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();
                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => result.to_string(),
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }

        Lang::JavaScript => {
            println!("Launch: {}", command_str);
            let code = std::fs::read_to_string("sandbox/src/solution.js").unwrap();
            let test = std::fs::read_to_string("sandbox/src/solution.test.js").unwrap();
            let code_and_test = format!("{}\n{}", code, test);
            let dependencies = std::fs::read_to_string("sandbox/package.json").unwrap();
            let src = format!("{}\n{}", dependencies, code_and_test);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts = command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    // check OS if windows then add ".cmd" to command name in command_parts[0]
                    let command = if cfg!(target_os = "windows") {
                        format!("{}.cmd", command_parts[0])
                    } else {
                        command_parts[0].to_string()
                    };
                    let output = std::process::Command::new(command)
                        .args(args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();
                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => result.to_string(),
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }
        Lang::TypeScript => {
            println!("Launch: {}", command_str);
            let code = std::fs::read_to_string("sandbox/src/solution.ts").unwrap();
            let test = std::fs::read_to_string("sandbox/src/solution.test.ts").unwrap();
            let config = std::fs::read_to_string("sandbox/tsconfig.json").unwrap();
            let code_and_test = format!("{}\n{}", code, test);
            let dependencies = std::fs::read_to_string("sandbox/package.json").unwrap();
            let src = format!("{}\n{}\n{}", dependencies, config, code_and_test);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts = command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    // check OS if windows then add ".cmd" to command name in command_parts[0]
                    let command = if cfg!(target_os = "windows") {
                        format!("{}.cmd", command_parts[0])
                    } else {
                        command_parts[0].to_string()
                    };
                    let output = std::process::Command::new(command)
                        .args(args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();
                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => result.to_string(),
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }
        Lang::Php => {
            println!("Launch: {}", command_str);
            let code = std::fs::read_to_string("sandbox/src/Solution.php").unwrap();
            let test = std::fs::read_to_string("sandbox/tests/SolutionTest.php").unwrap();
            let code_and_test = format!("{}\n{}", code, test);
            let dependencies = std::fs::read_to_string("sandbox/composer.json").unwrap();
            let src = format!("{}\n{}", dependencies, code_and_test);
            let key = format!("{}{}", command_str, src);
            let result_str_opt = cache.get(&key);
            let result_str = match result_str_opt {
                None => {
                    let command_parts = command_str.split(" ").collect::<Vec<&str>>();
                    let args = command_parts[1..].to_vec();
                    // check OS if windows then add ".cmd" to command name in command_parts[0]
                    let command = if cfg!(target_os = "windows") {
                        format!("{}.cmd", command_parts[0])
                    } else {
                        command_parts[0].to_string()
                    };
                    println!("ARGS - {:?}", args);

                    let output = std::process::Command::new(&command)
                        .args(&args)
                        .current_dir("sandbox")
                        .output()
                        .unwrap();

                    let exit_code = output.status.code().unwrap();
                    // let std_out = String::from_utf8(output.stdout).unwrap();
                    let std_err = String::from_utf8(output.stderr).unwrap();
                    let tuple: (i32, String) = (exit_code, std_err);
                    let json_str = serde_json::to_string(&tuple).unwrap();
                    cache.set(key, json_str.clone());
                    json_str
                }
                Some(result) => result.to_string(),
            };
            let parsed: (i32, String) = serde_json::from_str(&result_str).unwrap();

            let exit_code = parsed.0;
            let output = parsed.1;

            println!("Exit result: {}", exit_code == 0);
            if DEBUG {
                println!("Output: {}", output);
            }
            let exit_code_bool = exit_code == 0;
            (exit_code_bool, only_error_message(&output, exit_code))
        }
        _ => panic!("Unsupported language: {:?}", lang),
    }
}

pub fn create_project_rust(lang: &Lang, project: &Project) {
    match lang {
        Lang::Rust => {
            println!("Create sandbox project with");
            println!("{}\n{}", project.cargo_toml, project.lib_rs);
            let sandbox_path = "sandbox";
            let src_path = format!("{}/src", sandbox_path);
            let main_path = format!("{}/src/lib.rs", sandbox_path);
            let cargo_path = format!("{}/Cargo.toml", sandbox_path);
            if !std::path::Path::new(sandbox_path).exists() {
                std::fs::create_dir(sandbox_path).unwrap();
            } else {
                std::fs::remove_dir_all(sandbox_path).unwrap();
                std::fs::create_dir(sandbox_path).unwrap();
            }
            if !std::path::Path::new(&src_path).exists() {
                std::fs::create_dir(&src_path).unwrap();
            }
            std::fs::write(&main_path, &project.lib_rs).unwrap();
            std::fs::write(&cargo_path, &project.cargo_toml).unwrap();
        }
        _ => panic!("Unsupported language: {:?}", lang),
    }
}
pub fn create_project_java(project: &crate::java::Project) {
    println!("Create sandbox project with");
    println!(
        "{}\n{}\n{}",
        project.project_build_script, project.solution_code, project.test_code
    );
    let sandbox_path = "sandbox";

    let main_path = format!(
        "{}/src/main/java/com/example/solution/Solution.java",
        sandbox_path
    );
    let test_path = format!(
        "{}/src/test/java/com/example/solution/SolutionTest.java",
        sandbox_path
    );
    let pom_path = format!("{}/pom.xml", sandbox_path);
    if !std::path::Path::new(sandbox_path).exists() {
        std::fs::create_dir(sandbox_path).unwrap();
    } else {
        std::fs::remove_dir_all(sandbox_path).unwrap();
        std::fs::create_dir(sandbox_path).unwrap();
    }
    std::fs::create_dir_all(format!(
        "{}/src/main/java/com/example/solution",
        sandbox_path
    ))
    .unwrap();
    std::fs::create_dir_all(format!(
        "{}/src/test/java/com/example/solution",
        sandbox_path
    ))
    .unwrap();
    std::fs::write(&main_path, &project.solution_code).unwrap();
    std::fs::write(&test_path, &project.test_code).unwrap();
    std::fs::write(&pom_path, &project.project_build_script).unwrap();
}
pub fn create_project_scala(project: &crate::java::Project) {
    println!("Create sandbox project with");
    println!(
        "{}\n{}\n{}",
        project.project_build_script, project.solution_code, project.test_code
    );
    let sandbox_path = "sandbox";

    let main_path = format!("{}/src/main/scala/Solution.scala", sandbox_path);
    let test_path = format!("{}/src/test/scala/SolutionTest.scala", sandbox_path);
    let pom_path = format!("{}/build.sbt", sandbox_path);
    if !std::path::Path::new(sandbox_path).exists() {
        std::fs::create_dir(sandbox_path).unwrap();
    } else {
        std::fs::remove_dir_all(sandbox_path).unwrap();
        std::fs::create_dir(sandbox_path).unwrap();
    }
    std::fs::create_dir_all(format!("{}/src/main/scala", sandbox_path)).unwrap();
    std::fs::create_dir_all(format!("{}/src/test/scala", sandbox_path)).unwrap();
    std::fs::write(&main_path, &project.solution_code).unwrap();
    std::fs::write(&test_path, &project.test_code).unwrap();
    std::fs::write(&pom_path, &project.project_build_script).unwrap();
}
pub fn create_project_swift(project: &crate::java::Project) {
    println!("Create sandbox project with");
    println!(
        "{}\n{}\n{}",
        project.project_build_script, project.solution_code, project.test_code
    );
    let sandbox_path = "sandbox";

    let main_path = format!("{}/Sources/Solution/Solution.swift", sandbox_path);
    let test_path = format!("{}/Tests/SolutionTests/SolutionTests.swift", sandbox_path);
    let pom_path = format!("{}/Package.swift", sandbox_path);
    if !std::path::Path::new(sandbox_path).exists() {
        std::fs::create_dir(sandbox_path).unwrap();
    } else {
        std::fs::remove_dir_all(sandbox_path).unwrap();
        std::fs::create_dir(sandbox_path).unwrap();
    }
    std::fs::create_dir_all(format!("{}/Sources/Solution/", sandbox_path)).unwrap();
    std::fs::create_dir_all(format!("{}/Tests/SolutionTests", sandbox_path)).unwrap();
    std::fs::write(&main_path, &project.solution_code).unwrap();
    std::fs::write(&test_path, &project.test_code).unwrap();
    std::fs::write(&pom_path, &project.project_build_script).unwrap();
}
pub fn create_project_kotlin(project: &crate::java::Project) {
    println!("Create sandbox project with");
    println!(
        "{}\n{}\n{}",
        project.project_build_script, project.solution_code, project.test_code
    );
    let sandbox_path = "sandbox";

    let main_path = format!("{}/src/main/kotlin/Solution.kt", sandbox_path);
    let test_path = format!("{}/src/test/kotlin/SolutionTest.kt", sandbox_path);
    let pom_path = format!("{}/build.gradle", sandbox_path);
    if !std::path::Path::new(sandbox_path).exists() {
        std::fs::create_dir(sandbox_path).unwrap();
    } else {
        std::fs::remove_dir_all(sandbox_path).unwrap();
        std::fs::create_dir(sandbox_path).unwrap();
    }
    std::fs::create_dir_all(format!("{}/src/main/kotlin", sandbox_path)).unwrap();
    std::fs::create_dir_all(format!("{}/src/test/kotlin", sandbox_path)).unwrap();
    std::fs::write(&main_path, &project.solution_code).unwrap();
    std::fs::write(&test_path, &project.test_code).unwrap();
    std::fs::write(&pom_path, &project.project_build_script).unwrap();
}
pub fn create_project_python(project: &crate::java::Project) {
    println!("Create sandbox project with");
    println!(
        "{}\n{}\n{}",
        project.project_build_script, project.solution_code, project.test_code
    );
    let sandbox_path = "sandbox";

    let main_path = format!("{}/solution.py", sandbox_path);
    let test_path = format!("{}/test.py", sandbox_path);
    let pom_path = format!("{}/requirements.txt", sandbox_path);
    if !std::path::Path::new(sandbox_path).exists() {
        std::fs::create_dir(sandbox_path).unwrap();
    } else {
        std::fs::remove_dir_all(sandbox_path).unwrap();
        std::fs::create_dir(sandbox_path).unwrap();
    }
    std::fs::write(&main_path, &project.solution_code).unwrap();
    std::fs::write(&test_path, &project.test_code).unwrap();
    std::fs::write(&pom_path, &project.project_build_script).unwrap();
}

pub fn create_project_javascript(project: &crate::java::Project) {
    println!("Create sandbox project with");
    println!(
        "{}\n{}\n{}",
        project.project_build_script, project.solution_code, project.test_code
    );
    let sandbox_path = "sandbox";

    let main_path = format!("{}/src/solution.js", sandbox_path);
    let test_path = format!("{}/src/solution.test.js", sandbox_path);
    let pom_path = format!("{}/package.json", sandbox_path);
    if !std::path::Path::new(sandbox_path).exists() {
        std::fs::create_dir(sandbox_path).unwrap();
    } else {
        std::fs::remove_dir_all(sandbox_path).unwrap();
        std::fs::create_dir(sandbox_path).unwrap();
    }
    std::fs::create_dir_all(format!("{}/src", sandbox_path)).unwrap();
    std::fs::write(&main_path, &project.solution_code).unwrap();
    std::fs::write(&test_path, &project.test_code).unwrap();
    std::fs::write(&pom_path, &project.project_build_script).unwrap();
}

pub fn create_project_typescript(project: &crate::typescript::Project) {
    println!("Create sandbox project with");
    println!(
        "{}\n{}\n{}\n{}",
        project.project_build_script,
        project.typescript_config,
        project.solution_code,
        project.test_code
    );
    let sandbox_path = "sandbox";

    let main_path = format!("{}/src/solution.ts", sandbox_path);
    let test_path = format!("{}/src/solution.test.ts", sandbox_path);
    let pom_path = format!("{}/package.json", sandbox_path);
    let config_path = format!("{}/tsconfig.json", sandbox_path);
    if !std::path::Path::new(sandbox_path).exists() {
        std::fs::create_dir(sandbox_path).unwrap();
    } else {
        std::fs::remove_dir_all(sandbox_path).unwrap();
        std::fs::create_dir(sandbox_path).unwrap();
    }
    std::fs::create_dir_all(format!("{}/src", sandbox_path)).unwrap();
    std::fs::write(&main_path, &project.solution_code).unwrap();
    std::fs::write(&test_path, &project.test_code).unwrap();
    std::fs::write(&pom_path, &project.project_build_script).unwrap();
    std::fs::write(&config_path, &project.typescript_config).unwrap();
}

pub fn create_project_php(project: &crate::java::Project) {
    println!("Create sandbox project with");
    println!(
        "{}\n{}\n{}",
        project.project_build_script, project.solution_code, project.test_code
    );
    let sandbox_path = "sandbox";
    let main_path = format!("{}/src/Solution.php", sandbox_path);
    let test_path = format!("{}/tests/SolutionTest.php", sandbox_path);
    let pom_path = format!("{}/composer.json", sandbox_path);
    if !std::path::Path::new(sandbox_path).exists() {
        std::fs::create_dir(sandbox_path).unwrap();
    } else {
        std::fs::remove_dir_all(sandbox_path).unwrap();
        std::fs::create_dir(sandbox_path).unwrap();
    }
    std::fs::create_dir_all(format!("{}/src", sandbox_path)).unwrap();
    std::fs::create_dir_all(format!("{}/tests", sandbox_path)).unwrap();
    std::fs::write(&main_path, &project.solution_code).unwrap();
    std::fs::write(&test_path, &project.test_code).unwrap();
    std::fs::write(&pom_path, &project.project_build_script).unwrap();
}

fn only_error_message(output: &str, exit_code: i32) -> String {
    if exit_code == 0 {
        return "".to_string();
    } else {
        output.to_string()
    }
}
