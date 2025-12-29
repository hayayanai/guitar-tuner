/** i18n message schema type definition */
export interface MessageSchema {
  tuningShift: {
    halfStepDown: string;
    wholeStepDown: string;
    oneHalfStepDown: string;
    twoStepsDown: string;
    twoHalfStepsDown: string;
  };
  dropTuning: {
    dropDSharp: string;
    dropD: string;
    dropCSharp: string;
    dropC: string;
    dropB: string;
    enableLabel: string;
    enable: string;
    helpText: string;
    helpTextExample: string;
  };
  app: {
    title: string;
  };
  settings: {
    title: string;
    input: string;
    sensitivity: string;
    sensitivityHigh: string;
    sensitivityLow: string;
    referencePitch: string;
    dropTuning: string;
    trayIcon: string;
    theme: string;
    window: string;
    language: string;
  };
  status: {
    listening: string;
    starting: string;
    failed: string;
  };
  pitch: {
    standard: string;
    custom: string;
    customHint: string;
    shift: string;
    hz: string;
    errorRange: string;
  };
  trayIcon: {
    indicatorOnly: string;
    indicatorNote: string;
    indicatorCents: string;
  };
  theme: {
    system: string;
    light: string;
    dark: string;
  };
  window: {
    alwaysOnTop: string;
  };
  language: {
    en: string;
    ja: string;
  };
  update: {
    checking: string;
    available: string;
    version: string;
    downloading: string;
    updateAndInstall: string;
    later: string;
    close: string;
    error: string;
  };
}
