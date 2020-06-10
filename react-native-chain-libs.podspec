require "json"

package = JSON.parse(File.read(File.join(__dir__, "package.json")))

Pod::Spec.new do |s|
  s.name         = "react-native-chain-libs"
  s.version      = package["version"]
  s.summary      = package["description"]
  s.description  = <<-DESC
                  react-native-chain-libs
                   DESC
  s.homepage     = "https://github.com/Emurgo/react-native-chain-libs"
  s.license      = "MIT"
  # s.license    = { :type => "MIT", :file => "FILE_LICENSE" }
  s.authors      = { "Your Name" => "yourname@email.com" }
  s.platforms    = { :ios => "9.0", :tvos => "10.0" }
  s.source       = { :git => "https://github.com/Emurgo/react-native-chain-libs.git", :tag => "#{s.version}" }

  s.module_name = 'ChainLibs'

  s.source_files = "ios/**/*.{h,m,swift,sh}"
  s.requires_arc = true

  s.script_phase = {
    :name => "Build Rust Binary",
    :script => 'bash "${PODS_TARGET_SRCROOT}/ios/build.sh"',
    :execution_position => :before_compile
  }

  s.pod_target_xcconfig = {
    "HEADER_SEARCH_PATHS" => "$(CONFIGURATION_BUILD_DIR)",
    "OTHER_LIBTOOLFLAGS" => "-lreact_native_chain_libs",
    "ENABLE_BITCODE" => "NO"
  }

  s.preserve_paths = "rust/**/*"

  s.dependency "React"
end
