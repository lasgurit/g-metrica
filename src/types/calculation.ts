export type CalculationType =
  | "viga_fundacion"
  | "contrapisos"
  | "mamposteria"
  | "revoque_exterior"
  | "revoque_interior"
  | "techo"
  | "pisos"
  | "cielorraso"
  | "banio_estandar"
  | "cocina_estandar"
  | "electricidad"
  | "personalizado";

export interface Calculation {
  id: string;
  projectId: string;
  type: CalculationType;
  name: string;
  inputs: Record<string, number | string>;
  results: MaterialResult[];
  createdAt: Date;
  updatedAt: Date;
}

export interface MaterialResult {
  materialId: string;
  materialName: string;
  quantity: number;
  unit: string;
  editable?: boolean;
}

export interface CalculationSummary {
  projectId: string;
  materials: MaterialSummaryItem[];
  totalCalculations: number;
  generatedAt: Date;
}

export interface MaterialSummaryItem {
  materialId: string;
  materialName: string;
  totalQuantity: number;
  unit: string;
  usedInCalculations: string[];
}
