# src/onboarding/onboarding_wizard.nim
# First-boot onboarding wizard for SigmaOS
# Interactive setup for new users
#
# Features:
# - Welcome screen
# - Theme selection
# - App recommendations
# - Privacy settings
# - Agent training

type
  WizardStep* = enum
    Welcome
    SystemInfo
    ThemeSelection
    AppRecommendations
    PrivacySetup
    AgentTraining
    CloudSync
    Complete

  Theme* = enum
    DarkDefault
    LightDefault
    AutoWallpaper
    Custom

  AppCategory* = enum
    Development
    Productivity
    Media
    Gaming
    Utilities
    Social

  AppRecommendation* = object
    name*: string
    category*: AppCategory
    description*: string
    size_mb*: int
    selected*: bool

  PrivacyLevel* = enum
    Maximum      # No telemetry, no cloud sync
    Balanced     # Essential telemetry only
    Full         # All features enabled

  OnboardingConfig* = object
    username*: string
    theme*: Theme
    selectedApps*: seq[string]
    privacyLevel*: PrivacyLevel
    enableAgentAssistant*: bool
    enableCloudSync*: bool
    agentPersonality*: string

  OnboardingWizard* = object
    currentStep*: WizardStep
    config*: OnboardingConfig
    recommendations*: seq[AppRecommendation]
    completed*: bool

proc initWizard*(): OnboardingWizard =
  ## Initialize onboarding wizard
  result.currentStep = WizardStep.Welcome
  result.completed = false
  result.config = OnboardingConfig(
    username: "",
    theme: Theme.DarkDefault,
    selectedApps: @[],
    privacyLevel: PrivacyLevel.Balanced,
    enableAgentAssistant: true,
    enableCloudSync: false,
    agentPersonality: "helpful"
  )
  result.recommendations = @[]

proc generateAppRecommendations*(wizard: var OnboardingWizard) =
  ## Generate app recommendations based on user interests
  wizard.recommendations = @[
    AppRecommendation(
      name: "Sigma Code",
      category: AppCategory.Development,
      description: "AI-powered code editor with agent assistance",
      size_mb: 150,
      selected: false
    ),
    AppRecommendation(
      name: "Sigma Browser",
      category: AppCategory.Productivity,
      description: "Privacy-focused web browser with ad blocking",
      size_mb: 200,
      selected: true  # Recommended by default
    ),
    AppRecommendation(
      name: "Sigma Notes",
      category: AppCategory.Productivity,
      description: "Markdown note-taking with cloud sync",
      size_mb: 50,
      selected: false
    ),
    AppRecommendation(
      name: "Sigma Media",
      category: AppCategory.Media,
      description: "Universal media player (audio, video, streaming)",
      size_mb: 100,
      selected: true  # Recommended by default
    ),
    AppRecommendation(
      name: "Sigma Studio",
      category: AppCategory.Development,
      description: "Full IDE with debugging and profiling",
      size_mb: 300,
      selected: false
    ),
    AppRecommendation(
      name: "Sigma Files",
      category: AppCategory.Utilities,
      description: "Advanced file manager with tagging",
      size_mb: 80,
      selected: true  # Recommended by default
    ),
    AppRecommendation(
      name: "Sigma Terminal",
      category: AppCategory.Utilities,
      description: "GPU-accelerated terminal emulator",
      size_mb: 40,
      selected: true  # Recommended by default
    ),
    AppRecommendation(
      name: "Sigma Chat",
      category: AppCategory.Social,
      description: "Multi-protocol messenger (Matrix, XMPP, IRC)",
      size_mb: 120,
      selected: false
    )
  ]

proc nextStep*(wizard: var OnboardingWizard) =
  ## Advance to next wizard step
  case wizard.currentStep
  of WizardStep.Welcome:
    wizard.currentStep = WizardStep.SystemInfo
  of WizardStep.SystemInfo:
    wizard.currentStep = WizardStep.ThemeSelection
  of WizardStep.ThemeSelection:
    wizard.currentStep = WizardStep.AppRecommendations
    wizard.generateAppRecommendations()
  of WizardStep.AppRecommendations:
    wizard.currentStep = WizardStep.PrivacySetup
  of WizardStep.PrivacySetup:
    wizard.currentStep = WizardStep.AgentTraining
  of WizardStep.AgentTraining:
    wizard.currentStep = WizardStep.CloudSync
  of WizardStep.CloudSync:
    wizard.currentStep = WizardStep.Complete
    wizard.completed = true
  of WizardStep.Complete:
    discard

proc previousStep*(wizard: var OnboardingWizard) =
  ## Go back to previous wizard step
  case wizard.currentStep
  of WizardStep.Welcome:
    discard
  of WizardStep.SystemInfo:
    wizard.currentStep = WizardStep.Welcome
  of WizardStep.ThemeSelection:
    wizard.currentStep = WizardStep.SystemInfo
  of WizardStep.AppRecommendations:
    wizard.currentStep = WizardStep.ThemeSelection
  of WizardStep.PrivacySetup:
    wizard.currentStep = WizardStep.AppRecommendations
  of WizardStep.AgentTraining:
    wizard.currentStep = WizardStep.PrivacySetup
  of WizardStep.CloudSync:
    wizard.currentStep = WizardStep.AgentTraining
  of WizardStep.Complete:
    wizard.currentStep = WizardStep.CloudSync

proc setTheme*(wizard: var OnboardingWizard, theme: Theme) =
  ## Set user's preferred theme
  wizard.config.theme = theme

proc toggleApp*(wizard: var OnboardingWizard, appName: string) =
  ## Toggle app selection
  for i in 0..<wizard.recommendations.len:
    if wizard.recommendations[i].name == appName:
      wizard.recommendations[i].selected = not wizard.recommendations[i].selected
      
      if wizard.recommendations[i].selected:
        wizard.config.selectedApps.add(appName)
      else:
        let idx = wizard.config.selectedApps.find(appName)
        if idx >= 0:
          wizard.config.selectedApps.delete(idx)
      break

proc setPrivacyLevel*(wizard: var OnboardingWizard, level: PrivacyLevel) =
  ## Set privacy level
  wizard.config.privacyLevel = level
  
  # Adjust features based on privacy level
  case level
  of PrivacyLevel.Maximum:
    wizard.config.enableCloudSync = false
    wizard.config.enableAgentAssistant = false
  of PrivacyLevel.Balanced:
    wizard.config.enableCloudSync = false
    wizard.config.enableAgentAssistant = true
  of PrivacyLevel.Full:
    wizard.config.enableCloudSync = true
    wizard.config.enableAgentAssistant = true

proc setAgentPersonality*(wizard: var OnboardingWizard, personality: string) =
  ## Set agent assistant personality
  wizard.config.agentPersonality = personality

proc enableCloudSync*(wizard: var OnboardingWizard, enable: bool) =
  ## Enable/disable cloud sync
  wizard.config.enableCloudSync = enable

proc getProgress*(wizard: OnboardingWizard): int =
  ## Get wizard completion progress (0-100%)
  case wizard.currentStep
  of WizardStep.Welcome: 0
  of WizardStep.SystemInfo: 14
  of WizardStep.ThemeSelection: 28
  of WizardStep.AppRecommendations: 42
  of WizardStep.PrivacySetup: 57
  of WizardStep.AgentTraining: 71
  of WizardStep.CloudSync: 85
  of WizardStep.Complete: 100

proc getTotalSelectedAppsSize*(wizard: OnboardingWizard): int =
  ## Calculate total size of selected apps in MB
  result = 0
  for app in wizard.recommendations:
    if app.selected:
      result += app.size_mb

proc getStepDescription*(step: WizardStep): string =
  ## Get description for wizard step
  case step
  of WizardStep.Welcome:
    "Welcome to SigmaOS! Let's get you set up in just a few steps."
  of WizardStep.SystemInfo:
    "Let's gather some basic information about your system."
  of WizardStep.ThemeSelection:
    "Choose your visual theme. You can change this anytime."
  of WizardStep.AppRecommendations:
    "Here are some recommended apps. Select the ones you'd like to install."
  of WizardStep.PrivacySetup:
    "Configure your privacy settings. You have full control."
  of WizardStep.AgentTraining:
    "Train your AI agent assistant. What personality would you like?"
  of WizardStep.CloudSync:
    "Optionally enable cloud sync for settings and files."
  of WizardStep.Complete:
    "All done! Your system is ready to use."

proc applyConfiguration*(wizard: OnboardingWizard): bool =
  ## Apply configuration and finalize setup
  # In real implementation, would:
  # 1. Apply theme
  # 2. Install selected apps
  # 3. Configure privacy settings
  # 4. Setup agent assistant
  # 5. Enable cloud sync if requested
  
  if not wizard.completed:
    return false
  
  # For now, just return success
  return true

proc skipOnboarding*(wizard: var OnboardingWizard) =
  ## Skip onboarding and use defaults
  wizard.currentStep = WizardStep.Complete
  wizard.completed = true
  
  # Use default configuration
  wizard.config.theme = Theme.DarkDefault
  wizard.config.selectedApps = @["Sigma Browser", "Sigma Media", "Sigma Files", "Sigma Terminal"]
  wizard.config.privacyLevel = PrivacyLevel.Balanced
  wizard.config.enableAgentAssistant = true
  wizard.config.enableCloudSync = false

proc exportConfig*(wizard: OnboardingWizard): string =
  ## Export configuration as JSON-like string
  result = "{\n"
  result &= "  \"username\": \"" & wizard.config.username & "\",\n"
  result &= "  \"theme\": \"" & $wizard.config.theme & "\",\n"
  result &= "  \"selectedApps\": ["
  for i, app in wizard.config.selectedApps:
    result &= "\"" & app & "\""
    if i < wizard.config.selectedApps.len - 1:
      result &= ", "
  result &= "],\n"
  result &= "  \"privacyLevel\": \"" & $wizard.config.privacyLevel & "\",\n"
  result &= "  \"enableAgentAssistant\": " & $wizard.config.enableAgentAssistant & ",\n"
  result &= "  \"enableCloudSync\": " & $wizard.config.enableCloudSync & ",\n"
  result &= "  \"agentPersonality\": \"" & wizard.config.agentPersonality & "\"\n"
  result &= "}"

when isMainModule:
  # Test the wizard
  var wizard = initWizard()
  
  echo "Onboarding Wizard Test"
  echo "======================"
  
  # Step through wizard
  echo "\n1. Welcome"
  echo wizard.currentStep.getStepDescription()
  
  wizard.nextStep()
  echo "\n2. System Info"
  echo wizard.currentStep.getStepDescription()
  
  wizard.nextStep()
  echo "\n3. Theme Selection"
  echo wizard.currentStep.getStepDescription()
  wizard.setTheme(Theme.AutoWallpaper)
  
  wizard.nextStep()
  echo "\n4. App Recommendations"
  echo wizard.currentStep.getStepDescription()
  echo "Total selected apps size: ", wizard.getTotalSelectedAppsSize(), " MB"
  
  wizard.nextStep()
  echo "\n5. Privacy Setup"
  echo wizard.currentStep.getStepDescription()
  wizard.setPrivacyLevel(PrivacyLevel.Balanced)
  
  wizard.nextStep()
  echo "\n6. Agent Training"
  echo wizard.currentStep.getStepDescription()
  wizard.setAgentPersonality("friendly")
  
  wizard.nextStep()
  echo "\n7. Cloud Sync"
  echo wizard.currentStep.getStepDescription()
  wizard.enableCloudSync(false)
  
  wizard.nextStep()
  echo "\n8. Complete!"
  echo wizard.currentStep.getStepDescription()
  echo "Progress: ", wizard.getProgress(), "%"
  
  echo "\n\nFinal Configuration:"
  echo wizard.exportConfig()
  
  # Apply configuration
  if wizard.applyConfiguration():
    echo "\n✓ Configuration applied successfully!"
  else:
    echo "\n✗ Failed to apply configuration"
