export type TableListItem = {
  InstrumentID: number[]|String,
  InstrumentName: number[]|String,
  ExchangeID: number[]|String,
  ExpireDate: number[]|String,
  EndDelivDate: number[]|String,
  VolumeMultiple: number,
};

export type CommissionItem = {
  OpenRatioByMoney: number,
  OpenRatioByVolume: number,
  CloseRatioByMoney: number,
  CloseRatioByVolume: number,
  CloseTodayRatioByMoney: number,
  CloseTodayRatioByVolume: number,
}

export type MarginItem = {
  LongMarginRatioByMoney: number,
  LongMarginRatioByVolume: number,
  ShortMarginRatioByMoney: number,
  ShortMarginRatioByVolume: number,
}

export type TickItem = {
  LastPrice: number
}

export type TableListPagination = {
  total: number;
  pageSize: number;
  current: number;
};

export type TableListData = {
  list: TableListItem[];
  pagination: Partial<TableListPagination>;
};

export type TableListParams = {
  status?: string;
  name?: string;
  desc?: string;
  key?: number;
  pageSize?: number;
  currentPage?: number;
  filter?: Record<string, any[]>;
  sorter?: Record<string, any>;
};
