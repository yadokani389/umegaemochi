export type DisasterInfo = {
  title: string;
  description: string;
  warning: string;
  occurred: string;
};

export type Settings = {
  weather_city_id: string;
  atcoder_id: string;
  widget_interval: number;
  using_widgets: string[];
  auto_fullscreen: boolean;
};
